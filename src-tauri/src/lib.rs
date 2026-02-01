mod commands;
mod db;
mod errors;
mod models;
mod services;
mod utils;

use db::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            let database = Database::new(&app_data_dir)
                .expect("failed to initialize database");
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Projects
            commands::project::create_project,
            commands::project::list_projects,
            commands::project::get_project,
            commands::project::delete_project,
            commands::project::validate_path,
            // Specs
            commands::spec::upload_spec,
            commands::spec::get_spec,
            commands::spec::list_specs,
            commands::spec::delete_spec,
            commands::spec::reparse_spec,
            commands::spec::read_file_content,
            // Test Generation
            commands::test_gen::generate_tests,
            commands::test_gen::get_generated_tests,
            commands::test_gen::get_all_generated_tests,
            commands::test_gen::save_test_to_disk,
            commands::test_gen::save_settings,
            commands::test_gen::load_settings,
            // Test Execution
            commands::test_exec::execute_tests,
            commands::test_exec::get_test_results,
            commands::test_exec::get_test_result,
            // Reports
            commands::report::generate_alignment_report,
            commands::report::get_alignment_report,
            commands::report::list_reports,
            commands::report::export_report,
            // Git
            commands::git::get_repo_info,
            commands::git::get_changed_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
