use std::process::Command;
use std::path::Path;
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;
use crate::errors::AppError;

const TEST_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub status: String,  // "passed", "failed", "error"
    pub execution_time_ms: i64,
    pub stdout: String,
    pub stderr: String,
}

fn run_with_timeout(mut child: std::process::Child, timeout: Duration, start: Instant) -> ExecutionResult {
    let elapsed_fn = || start.elapsed().as_millis() as i64;

    // Read stdout/stderr in separate threads to avoid pipe deadlocks
    let stdout_handle = child.stdout.take().map(|s| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            std::io::Read::read_to_end(&mut std::io::BufReader::new(s), &mut buf).ok();
            buf
        })
    });
    let stderr_handle = child.stderr.take().map(|s| {
        std::thread::spawn(move || {
            let mut buf = Vec::new();
            std::io::Read::read_to_end(&mut std::io::BufReader::new(s), &mut buf).ok();
            buf
        })
    });

    match child.wait_timeout(timeout) {
        Ok(Some(status)) => {
            let stdout = stdout_handle.and_then(|h| h.join().ok()).unwrap_or_default();
            let stderr = stderr_handle.and_then(|h| h.join().ok()).unwrap_or_default();
            ExecutionResult {
                status: if status.success() { "passed" } else { "failed" }.to_string(),
                execution_time_ms: elapsed_fn(),
                stdout: String::from_utf8_lossy(&stdout).to_string(),
                stderr: String::from_utf8_lossy(&stderr).to_string(),
            }
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            ExecutionResult {
                status: "error".to_string(),
                execution_time_ms: elapsed_fn(),
                stdout: String::new(),
                stderr: format!("Test timed out after {}s", timeout.as_secs()),
            }
        }
        Err(e) => {
            ExecutionResult {
                status: "error".to_string(),
                execution_time_ms: elapsed_fn(),
                stdout: String::new(),
                stderr: format!("Failed to wait for process: {}", e),
            }
        }
    }
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
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    match result {
        Ok(child) => Ok(run_with_timeout(child, TEST_TIMEOUT, start)),
        Err(e) => Ok(ExecutionResult {
            status: "error".to_string(),
            execution_time_ms: start.elapsed().as_millis() as i64,
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
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn();

    match result {
        Ok(child) => Ok(run_with_timeout(child, TEST_TIMEOUT, start)),
        Err(e) => Ok(ExecutionResult {
            status: "error".to_string(),
            execution_time_ms: start.elapsed().as_millis() as i64,
            stdout: String::new(),
            stderr: format!("Failed to execute PyTest: {}", e),
        }),
    }
}

fn find_python() -> String {
    // Try python3 first, fall back to python
    if let Ok(output) = Command::new("python3").arg("--version").output() {
        if output.status.success() {
            return "python3".to_string();
        }
    }
    "python".to_string()
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
