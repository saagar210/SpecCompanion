use std::process::Command;
use std::path::Path;
use std::time::Instant;
use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub status: String,  // "passed", "failed", "error"
    pub execution_time_ms: i64,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_jest_test(test_file: &str, working_dir: &str) -> Result<ExecutionResult, AppError> {
    let start = Instant::now();

    let npx = if cfg!(target_os = "windows") { "npx.cmd" } else { "npx" };
    let result = Command::new(npx)
        .arg("jest")
        .arg(test_file)
        .arg("--no-coverage")
        .arg("--verbose")
        .current_dir(working_dir)
        .output();

    let elapsed = start.elapsed().as_millis() as i64;

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status = if output.status.success() {
                "passed"
            } else {
                "failed"
            };
            Ok(ExecutionResult {
                status: status.to_string(),
                execution_time_ms: elapsed,
                stdout,
                stderr,
            })
        }
        Err(e) => Ok(ExecutionResult {
            status: "error".to_string(),
            execution_time_ms: elapsed,
            stdout: String::new(),
            stderr: format!("Failed to execute Jest: {}", e),
        }),
    }
}

pub fn run_pytest_test(test_file: &str, working_dir: &str) -> Result<ExecutionResult, AppError> {
    let start = Instant::now();

    let python = find_python();
    let result = Command::new(&python)
        .arg("-m")
        .arg("pytest")
        .arg(test_file)
        .arg("-v")
        .current_dir(working_dir)
        .output();

    let elapsed = start.elapsed().as_millis() as i64;

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let status = if output.status.success() {
                "passed"
            } else {
                "failed"
            };
            Ok(ExecutionResult {
                status: status.to_string(),
                execution_time_ms: elapsed,
                stdout,
                stderr,
            })
        }
        Err(e) => Ok(ExecutionResult {
            status: "error".to_string(),
            execution_time_ms: elapsed,
            stdout: String::new(),
            stderr: format!("Failed to execute PyTest: {}", e),
        }),
    }
}

fn find_python() -> String {
    // Try python3 first, fall back to python
    if Command::new("python3").arg("--version").output().is_ok() {
        "python3".to_string()
    } else {
        "python".to_string()
    }
}

#[allow(dead_code)]
pub fn check_framework_available(framework: &str, working_dir: &str) -> bool {
    let dir = Path::new(working_dir);
    match framework {
        "jest" => {
            // Check if node_modules/jest exists or package.json has jest
            dir.join("node_modules/.bin/jest").exists()
                || dir.join("node_modules/jest").exists()
        }
        "pytest" => {
            Command::new(find_python())
                .args(["-m", "pytest", "--version"])
                .current_dir(working_dir)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
        _ => false,
    }
}
