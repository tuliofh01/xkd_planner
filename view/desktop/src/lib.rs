use controllers::{AppServices, PtimerSession, SlogRecord, Task, TimelineEntry, User};

// HINT: every command calls one controller handler, then maps the error with
// `.map_err(|e| e.to_string())` so Tauri can serialise it.

#[tauri::command]
async fn register(
    _state: tauri::State<'_, AppServices>,
    _username: String,
    _password: String,
) -> Result<User, String> {
    todo!("state.auth.register(&username, &password)")
}

#[tauri::command]
async fn login(
    _state: tauri::State<'_, AppServices>,
    _username: String,
    _password: String,
) -> Result<User, String> {
    todo!("state.auth.login(&username, &password)")
}

#[tauri::command]
async fn list_users(_state: tauri::State<'_, AppServices>) -> Result<Vec<User>, String> {
    todo!("state.db.list_users()")
}

#[tauri::command]
async fn list_tasks(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
) -> Result<Vec<Task>, String> {
    todo!("controllers::controllers::daily_ctrl::list(&state.db, &user_id)")
}

#[tauri::command]
async fn create_task(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
    _title: String,
    _description: String,
    _due_at: Option<String>,
) -> Result<Task, String> {
    todo!("daily_ctrl::create(&state.db, &user_id, &title, &description, due_at.as_deref())")
}

#[tauri::command]
async fn complete_task(
    _state: tauri::State<'_, AppServices>,
    _id: String,
) -> Result<Task, String> {
    todo!("daily_ctrl::complete(&state.db, &id)")
}

#[tauri::command]
async fn delete_task(_state: tauri::State<'_, AppServices>, _id: String) -> Result<(), String> {
    todo!("daily_ctrl::remove(&state.db, &id)")
}

#[tauri::command]
async fn list_logs(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
) -> Result<Vec<SlogRecord>, String> {
    todo!("log_ctrl::list(&state.db, &user_id)")
}

#[tauri::command]
async fn write_log(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
    _task_id: Option<String>,
    _message: String,
) -> Result<SlogRecord, String> {
    todo!("log_ctrl::write(&state.db, &user_id, task_id.as_deref(), &message)")
}

#[tauri::command]
async fn start_timer(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
    _task_id: String,
) -> Result<PtimerSession, String> {
    todo!("ptimer_ctrl::start(&state.db, &user_id, &task_id)")
}

#[tauri::command]
async fn stop_timer(
    _state: tauri::State<'_, AppServices>,
    _id: String,
) -> Result<PtimerSession, String> {
    todo!("ptimer_ctrl::stop(&state.db, &id)")
}

#[tauri::command]
async fn list_timers(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
) -> Result<Vec<PtimerSession>, String> {
    todo!("ptimer_ctrl::list(&state.db, &user_id)")
}

#[tauri::command]
async fn add_timeline_entry(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
    _task_id: String,
    _starts_at: String,
    _ends_at: Option<String>,
) -> Result<TimelineEntry, String> {
    todo!("timeline_ctrl::add(&state.db, &user_id, &task_id, &starts_at, ends_at.as_deref())")
}

#[tauri::command]
async fn list_timeline(
    _state: tauri::State<'_, AppServices>,
    _user_id: String,
) -> Result<Vec<TimelineEntry>, String> {
    todo!("timeline_ctrl::list(&state.db, &user_id)")
}

/// Runs the Tauri application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let services = tauri::async_runtime::block_on(
        AppServices::new("sqlite:xkdplanner.db"),
    )
    .expect("failed to initialise application services");

    tauri::Builder::default()
        .manage(services)
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            list_users,
            list_tasks,
            create_task,
            complete_task,
            delete_task,
            list_logs,
            write_log,
            start_timer,
            stop_timer,
            list_timers,
            add_timeline_entry,
            list_timeline,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
