use std::path::Path;
use crate::errors::AppError;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CodeSymbol {
    pub name: String,
    pub kind: String, // "function", "class", "method"
    pub file_path: String,
}

const IGNORE_DIRS: &[&str] = &[
    "node_modules", ".git", "dist", "build", "target", ".next",
    "__pycache__", ".venv", "venv", ".tox", "coverage", ".nyc_output",
];

const SOURCE_EXTENSIONS: &[&str] = &[
    "ts", "tsx", "js", "jsx", "py", "rs", "go", "java", "rb", "cs",
];

const MAX_DEPTH: usize = 12;

pub fn scan_codebase(root: &str, exclusions: &[String]) -> Result<Vec<CodeSymbol>, AppError> {
    let root_path = Path::new(root);
    if !root_path.exists() || !root_path.is_dir() {
        return Err(AppError::InvalidInput(format!("Invalid codebase path: {}", root)));
    }

    let mut symbols = Vec::new();
    walk_dir(root_path, root_path, &mut symbols, exclusions, 0)?;
    Ok(symbols)
}

fn walk_dir(
    dir: &Path,
    root: &Path,
    symbols: &mut Vec<CodeSymbol>,
    exclusions: &[String],
    depth: usize,
) -> Result<(), AppError> {
    if depth > MAX_DEPTH {
        return Ok(());
    }
    let entries = std::fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if IGNORE_DIRS.contains(&name.as_str()) || exclusions.contains(&name) {
            continue;
        }

        if path.is_dir() {
            walk_dir(&path, root, symbols, exclusions, depth + 1)?;
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if SOURCE_EXTENSIONS.contains(&ext) {
                // Skip files larger than 1 MB to avoid reading generated/bundled files
                let too_large = std::fs::metadata(&path)
                    .map(|m| m.len() > 1_024_000)
                    .unwrap_or(false);
                if too_large {
                    continue;
                }
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let rel_path = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().to_string();
                    extract_symbols(&content, &rel_path, ext, symbols);
                }
            }
        }
    }
    Ok(())
}

fn extract_symbols(content: &str, file_path: &str, ext: &str, symbols: &mut Vec<CodeSymbol>) {
    match ext {
        "ts" | "tsx" | "js" | "jsx" => extract_js_ts_symbols(content, file_path, symbols),
        "py" => extract_python_symbols(content, file_path, symbols),
        "rs" => extract_rust_symbols(content, file_path, symbols),
        "go" => extract_go_symbols(content, file_path, symbols),
        "java" => extract_java_symbols(content, file_path, symbols),
        "rb" => extract_ruby_symbols(content, file_path, symbols),
        "cs" => extract_csharp_symbols(content, file_path, symbols),
        _ => {}
    }
}

fn extract_js_ts_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        // function declarations
        if let Some(name) = extract_after_keyword(trimmed, "function ") {
            symbols.push(CodeSymbol { name, kind: "function".into(), file_path: file_path.into() });
        }
        // class declarations
        if let Some(name) = extract_after_keyword(trimmed, "class ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        // const arrow functions: const foo = (...) =>
        if (trimmed.starts_with("export const ") || trimmed.starts_with("const "))
            && (trimmed.contains("=>") || trimmed.contains("= function"))
        {
            let after_const = if trimmed.starts_with("export ") {
                &trimmed[13..] // after "export const "
            } else {
                &trimmed[6..] // after "const "
            };
            if let Some(pos) = after_const.find(|c: char| !c.is_alphanumeric() && c != '_') {
                let name = after_const[..pos].to_string();
                if !name.is_empty() {
                    symbols.push(CodeSymbol { name, kind: "function".into(), file_path: file_path.into() });
                }
            }
        }
    }
}

fn extract_python_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_after_keyword(trimmed, "def ") {
            let kind = if line.starts_with("    ") || line.starts_with('\t') {
                "method"
            } else {
                "function"
            };
            symbols.push(CodeSymbol { name, kind: kind.into(), file_path: file_path.into() });
        }
        if let Some(name) = extract_after_keyword(trimmed, "class ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
    }
}

fn extract_rust_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_after_keyword(trimmed, "fn ") {
            symbols.push(CodeSymbol { name, kind: "function".into(), file_path: file_path.into() });
        }
        if let Some(name) = extract_after_keyword(trimmed, "struct ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        if let Some(name) = extract_after_keyword(trimmed, "impl ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
    }
}

fn extract_go_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_after_keyword(trimmed, "func ") {
            symbols.push(CodeSymbol { name, kind: "function".into(), file_path: file_path.into() });
        }
        if let Some(name) = extract_after_keyword(trimmed, "type ") {
            if trimmed.contains(" struct") || trimmed.contains(" interface") {
                symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
            }
        }
    }
}

fn extract_java_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_after_keyword(trimmed, "class ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        if let Some(name) = extract_after_keyword(trimmed, "interface ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        if looks_like_java_method(trimmed) {
            if let Some(name) = extract_method_name_before_paren(trimmed) {
                symbols.push(CodeSymbol { name, kind: "method".into(), file_path: file_path.into() });
            }
        }
    }
}

fn extract_ruby_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_after_keyword(trimmed, "class ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        if let Some(rest) = trimmed.strip_prefix("def ") {
            let method = rest.trim_start();
            let name_part = method.split_whitespace().next().unwrap_or("");
            let name_part = name_part.split('(').next().unwrap_or("");
            let name = name_part.rsplit('.').next().unwrap_or("").to_string();
            if !name.is_empty() {
                symbols.push(CodeSymbol { name, kind: "method".into(), file_path: file_path.into() });
            }
        }
    }
}

fn extract_csharp_symbols(content: &str, file_path: &str, symbols: &mut Vec<CodeSymbol>) {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(name) = extract_after_keyword(trimmed, "class ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        if let Some(name) = extract_after_keyword(trimmed, "interface ") {
            symbols.push(CodeSymbol { name, kind: "class".into(), file_path: file_path.into() });
        }
        if looks_like_csharp_method(trimmed) {
            if let Some(name) = extract_method_name_before_paren(trimmed) {
                symbols.push(CodeSymbol { name, kind: "method".into(), file_path: file_path.into() });
            }
        }
    }
}

fn looks_like_java_method(line: &str) -> bool {
    line.contains('(')
        && line.contains(')')
        && (line.ends_with('{') || line.ends_with(";") || line.contains(" throws "))
        && !line.contains(" class ")
        && !line.starts_with("if ")
        && !line.starts_with("for ")
        && !line.starts_with("while ")
        && !line.starts_with("switch ")
}

fn looks_like_csharp_method(line: &str) -> bool {
    line.contains('(')
        && line.contains(')')
        && (line.ends_with('{') || line.ends_with("=>") || line.contains(" => "))
        && !line.contains(" class ")
        && !line.starts_with("if ")
        && !line.starts_with("for ")
        && !line.starts_with("while ")
        && !line.starts_with("switch ")
}

fn extract_method_name_before_paren(line: &str) -> Option<String> {
    let paren = line.find('(')?;
    let before = line[..paren].trim();
    let candidate = before.split_whitespace().last()?;
    let cleaned = candidate.trim_matches(|c: char| c == '<' || c == '>' || c == ':' || c == ',');
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned.to_string())
    }
}

fn extract_after_keyword(line: &str, keyword: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix(keyword)
        .or_else(|| {
            // Also match "pub fn ", "async fn ", "export function ", etc.
            let idx = line.find(keyword)?;
            Some(&line[idx + keyword.len()..])
        })
    {
        let name: String = rest.chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() {
            return Some(name);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_symbols_for_supported_non_rust_languages() {
        let go = r#"
            package demo
            type User struct {}
            func BuildUser() {}
        "#;
        let java = r#"
            public class AccountService {
                public void createAccount(String id) {}
            }
        "#;
        let ruby = r#"
            class AccountService
              def self.build
              end
            end
        "#;
        let csharp = r#"
            public class BillingService {
                public void ChargeCustomer(string id) { }
            }
        "#;

        let mut symbols = Vec::new();
        extract_symbols(go, "main.go", "go", &mut symbols);
        extract_symbols(java, "Main.java", "java", &mut symbols);
        extract_symbols(ruby, "main.rb", "rb", &mut symbols);
        extract_symbols(csharp, "Main.cs", "cs", &mut symbols);

        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"User"));
        assert!(names.contains(&"BuildUser"));
        assert!(names.contains(&"AccountService"));
        assert!(names.contains(&"createAccount"));
        assert!(names.contains(&"build"));
        assert!(names.contains(&"BillingService"));
        assert!(names.contains(&"ChargeCustomer"));
    }
}
