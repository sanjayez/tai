#[tauri::command]
fn app_status() -> app_core::AppStatus {
    app_core::AppStatus::initial()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_status])
        .run(tauri::generate_context!())
        .expect("failed to run Tally AI Companion desktop app");
}

fn main() {
    run();
}

