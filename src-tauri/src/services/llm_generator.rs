use crate::errors::AppError;
use crate::models::spec::Requirement;
use crate::services::codebase_scanner::CodeSymbol;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";
const FALLBACK_MODELS: &[&str] = &[
    "claude-sonnet-4-20250514",
    "claude-sonnet-3-5-20241022",
    "claude-3-5-sonnet-20241022",
];

pub async fn generate_test_with_llm(
    api_key: &str,
    requirement: &Requirement,
    framework: &str,
    symbols: &[CodeSymbol],
) -> Result<String, AppError> {
    generate_test_with_model(api_key, requirement, framework, symbols, None).await
}

pub async fn generate_test_with_model(
    api_key: &str,
    requirement: &Requirement,
    framework: &str,
    symbols: &[CodeSymbol],
    model_override: Option<String>,
) -> Result<String, AppError> {
    let context = build_context(symbols);
    let prompt = build_prompt_with_examples(requirement, framework, &context);

    let model = model_override.unwrap_or_else(|| DEFAULT_MODEL.to_string());

    // Try primary model first, then fallbacks
    let mut last_error = None;
    let models_to_try = if FALLBACK_MODELS.contains(&model.as_str()) {
        vec![model.clone()]
    } else {
        let mut models = vec![model.clone()];
        models.extend(FALLBACK_MODELS.iter().map(|s| s.to_string()));
        models
    };

    for (index, try_model) in models_to_try.iter().enumerate() {
        match try_api_call(api_key, try_model, &prompt).await {
            Ok(test_code) => {
                if index > 0 {
                    eprintln!("Note: Primary model failed, used fallback: {}", try_model);
                }
                return Ok(test_code);
            }
            Err(e) => {
                eprintln!("Model {} failed: {}", try_model, e);
                last_error = Some(e);
                if index < models_to_try.len() - 1 {
                    continue; // Try next model
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| AppError::General("All models failed".to_string())))
}

async fn try_api_call(api_key: &str, model: &str, prompt: &str) -> Result<String, AppError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(90))
        .build()
        .map_err(AppError::Http)?;

    let request = ClaudeRequest {
        model: model.to_string(),
        max_tokens: 2048,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    let start_time = std::time::Instant::now();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await?;

    let elapsed = start_time.elapsed();
    eprintln!("API call to {} took {:?}", model, elapsed);

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::General(format!(
            "Claude API error ({}): {}",
            status, body
        )));
    }

    let claude_response: ClaudeResponse = response.json().await?;
    let test_code = claude_response
        .content
        .into_iter()
        .filter_map(|block| block.text)
        .collect::<Vec<_>>()
        .join("\n");

    let code = extract_code_block(&test_code).unwrap_or(test_code);
    Ok(code)
}

fn build_context(symbols: &[CodeSymbol]) -> String {
    if symbols.is_empty() {
        return String::from("No codebase context available.");
    }

    let mut context = String::from("Codebase symbols:\n");
    for sym in symbols.iter().take(30) {
        context.push_str(&format!("- {} {} (in {})\n", sym.kind, sym.name, sym.file_path));
    }
    context
}

fn build_prompt_with_examples(requirement: &Requirement, framework: &str, context: &str) -> String {
    let (framework_info, example) = match framework {
        "jest" => (
            "Jest (JavaScript/TypeScript testing framework)",
            r#"
Example Jest test structure:
```typescript
// REQ-001: User authentication
describe('User Authentication', () => {
  it('should authenticate user with valid credentials', () => {
    // Arrange
    const mockUser = { email: 'test@example.com', password: 'validPass123' };
    const authService = new AuthService();

    // Act
    const result = authService.login(mockUser.email, mockUser.password);

    // Assert
    expect(result.success).toBe(true);
    expect(result.user).toBeDefined();
    expect(result.user.email).toBe(mockUser.email);
  });

  it('should reject invalid credentials', () => {
    const authService = new AuthService();
    const result = authService.login('test@example.com', 'wrongPass');
    expect(result.success).toBe(false);
    expect(result.error).toBe('Invalid credentials');
  });
});
```"#
        ),
        "pytest" => (
            "pytest (Python testing framework)",
            r#"
Example pytest test structure:
```python
# REQ-001: User authentication

class TestUserAuthentication:
    def test_authenticate_with_valid_credentials(self):
        """Should authenticate user with valid email and password."""
        # Arrange
        auth_service = AuthService()
        email = "test@example.com"
        password = "validPass123"

        # Act
        result = auth_service.login(email, password)

        # Assert
        assert result.success is True
        assert result.user is not None
        assert result.user.email == email

    def test_reject_invalid_credentials(self):
        """Should reject authentication with wrong password."""
        auth_service = AuthService()
        result = auth_service.login("test@example.com", "wrongPass")
        assert result.success is False
        assert result.error == "Invalid credentials"
```"#
        ),
        _ => ("Unknown framework", ""),
    };

    format!(
        r#"Generate a test for the following requirement. Output ONLY the test code in a markdown code block, no explanations before or after.

**Requirement Details:**
- Description: {}
- Section: {}
- Type: {}
- Priority: {}

**Test Framework:** {}

**Codebase Context:**
{}

{}

**Requirements for generated test:**
1. Clear arrange/act/assert structure (AAA pattern)
2. Meaningful assertions that actually test the requirement (not placeholders like `expect(true).toBe(true)`)
3. Traceability comment at top linking to requirement ID or description
4. Cover main happy path + at least one edge case or error scenario
5. Use realistic mock data and object names matching the domain
6. Include descriptive test names that explain what is being tested
7. Follow framework best practices and conventions
8. Keep tests focused and readable (each test should verify one behavior)

Output the complete, ready-to-run test code:"#,
        requirement.description,
        requirement.section,
        requirement.req_type,
        requirement.priority,
        framework_info,
        context,
        example
    )
}

fn build_prompt(requirement: &Requirement, framework: &str, context: &str) -> String {
    build_prompt_with_examples(requirement, framework, context)
}

fn extract_code_block(text: &str) -> Option<String> {
    // Try to find ```typescript, ```javascript, ```python, or generic ``` blocks
    let patterns = ["```typescript", "```javascript", "```python", "```js", "```ts", "```py", "```"];
    for pattern in patterns {
        if let Some(start) = text.find(pattern) {
            let code_start = start + pattern.len();
            // Skip to next line
            let code_start = text[code_start..].find('\n').map(|i| code_start + i + 1)?;
            let code_end = text[code_start..].find("```").map(|i| code_start + i)?;
            return Some(text[code_start..code_end].trim().to_string());
        }
    }
    None
}
