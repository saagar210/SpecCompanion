import { invoke } from "@tauri-apps/api/core";
import type {
  Project,
  CreateProjectRequest,
  ProjectWithStats,
  Spec,
  Requirement,
  ParsedSpec,
  GeneratedTest,
  GenerateTestsRequest,
  TestResult,
  AlignmentReport,
  AlignmentReportWithMismatches,
  AppSettings,
} from "./types";

// Project commands
export const createProject = (req: CreateProjectRequest) =>
  invoke<Project>("create_project", { request: req });

export const listProjects = () =>
  invoke<ProjectWithStats[]>("list_projects");

export const getProject = (id: string) =>
  invoke<ProjectWithStats>("get_project", { id });

export const deleteProject = (id: string) =>
  invoke<void>("delete_project", { id });

export const validatePath = (path: string) =>
  invoke<boolean>("validate_path", { path });

// Spec commands
export const uploadSpec = (projectId: string, filename: string, content: string) =>
  invoke<ParsedSpec>("upload_spec", { project_id: projectId, filename, content });

export const getSpec = (id: string) =>
  invoke<ParsedSpec>("get_spec", { id });

export const listSpecs = (projectId: string) =>
  invoke<Spec[]>("list_specs", { project_id: projectId });

export const deleteSpec = (id: string) =>
  invoke<void>("delete_spec", { id });

export const reparseSpec = (id: string) =>
  invoke<Requirement[]>("reparse_spec", { id });

export const readFileContent = (path: string) =>
  invoke<string>("read_file_content", { path });

// Test generation commands
export const generateTests = (req: GenerateTestsRequest) =>
  invoke<GeneratedTest[]>("generate_tests", { request: req });

export const getGeneratedTests = (requirementId: string) =>
  invoke<GeneratedTest[]>("get_generated_tests", { requirement_id: requirementId });

export const getAllGeneratedTests = (projectId: string) =>
  invoke<GeneratedTest[]>("get_all_generated_tests", { project_id: projectId });

export const saveTestToDisk = (testId: string, path: string) =>
  invoke<string>("save_test_to_disk", { test_id: testId, path });

// Settings commands
export const saveSettings = (settings: AppSettings) =>
  invoke<void>("save_settings", { settings });

export const loadSettings = () =>
  invoke<AppSettings>("load_settings");

// Test execution commands
export const executeTests = (projectId: string, testIds: string[]) =>
  invoke<TestResult[]>("execute_tests", { project_id: projectId, test_ids: testIds });

export const getTestResults = (projectId: string) =>
  invoke<TestResult[]>("get_test_results", { project_id: projectId });

export const getTestResult = (id: string) =>
  invoke<TestResult>("get_test_result", { id });

// Report commands
export const generateAlignmentReport = (projectId: string) =>
  invoke<AlignmentReportWithMismatches>("generate_alignment_report", { project_id: projectId });

export const getAlignmentReport = (id: string) =>
  invoke<AlignmentReportWithMismatches>("get_alignment_report", { id });

export const listReports = (projectId: string) =>
  invoke<AlignmentReport[]>("list_reports", { project_id: projectId });

export const exportReport = (reportId: string, format: "json" | "html" | "csv") =>
  invoke<string>("export_report", { report_id: reportId, format });
