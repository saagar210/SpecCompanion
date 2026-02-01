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
