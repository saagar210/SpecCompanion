// Project types
export interface Project {
  id: string;
  name: string;
  codebase_path: string;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectRequest {
  name: string;
  codebase_path: string;
}

export interface ProjectWithStats extends Project {
  spec_count: number;
  coverage_percent: number | null;
  last_run_at: string | null;
}

// Spec types
export interface Spec {
  id: string;
  project_id: string;
  filename: string;
  content: string;
  parsed_at: string | null;
  created_at: string;
}

export interface Requirement {
  id: string;
  spec_id: string;
  section: string;
  description: string;
  req_type: "functional" | "non_functional" | "constraint";
  priority: "high" | "medium" | "low";
}

export interface ParsedSpec {
  spec: Spec;
  requirements: Requirement[];
}

// Test generation types
export interface GeneratedTest {
  id: string;
  requirement_id: string;
  framework: "jest" | "pytest";
  code: string;
  generation_mode: "template" | "llm";
  file_path: string | null;
  created_at: string;
}

export interface GenerateTestsRequest {
  requirement_ids: string[];
  framework: "jest" | "pytest";
  mode: "template" | "llm";
  project_id: string;
}

// Test execution types
export interface TestResult {
  id: string;
  generated_test_id: string;
  status: "passed" | "failed" | "skipped" | "error";
  execution_time_ms: number;
  stdout: string;
  stderr: string;
  executed_at: string;
}

export interface TestProgress {
  total: number;
  completed: number;
  current_test: string;
  status: "running" | "completed" | "error";
}

// Report types
export interface AlignmentReport {
  id: string;
  project_id: string;
  coverage_percent: number;
  total_requirements: number;
  covered_requirements: number;
  generated_at: string;
}

export interface Mismatch {
  id: string;
  report_id: string;
  requirement_id: string;
  spec_section: string;
  code_element: string | null;
  mismatch_type: "not_implemented" | "test_failing" | "no_test_generated" | "partial_coverage";
  details: string;
}

export interface AlignmentReportWithMismatches extends AlignmentReport {
  mismatches: Mismatch[];
}

// Settings
export interface AppSettings {
  api_key: string;
  default_framework: "jest" | "pytest";
  default_mode: "template" | "llm";
  scan_exclusions: string[];
}
