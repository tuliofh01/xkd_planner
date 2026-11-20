// ============================================================================
//  xkdPlanner — Tauri application entry point
// ============================================================================
//
//  This tiny file is the binary target of the `app` package. Its only job is
//  to hand control over to `app_lib::run()`, which lives in `src/lib.rs`.
//
//  Why split main.rs and lib.rs?
//  -----------------------------
//  * main.rs  -> the executable. Cargo builds one binary per `[[bin]]` target;
//               by default that target is `src/main.rs`.
//  * lib.rs   -> the library (`app_lib`). Tauri needs a library target because
//               mobile platforms embed the app as a library, not a binary.
//
//  Keeping nearly all code in lib.rs means the same logic is reused by both
//  desktop and mobile builds.

// This attribute is Windows-specific and easy to overlook:
//
//   - In debug builds (`debug_assertions` is true) it is NOT applied, so a
//     console window appears and you can read `println!` / log output.
//   - In release builds it hides that console window, so users only see the
//     Tauri webview window. Without it, a black terminal pops up next to the
//     app. Tauri's generator includes it for this reason — do not remove it.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // `app_lib` is the name declared in Cargo.toml under [lib]:
    //
    //     [lib]
    //     name = "app_lib"
    //
    // Rust crates can be imported by any target in the same package, so the
    // binary can call the library's public `run()` function directly.
    //
    // Inside run() (see src/lib.rs) three things happen:
    //   1. AppServices is created — the database connection is opened and
    //      the shared state is registered with `.manage(...)`.
    //   2. All `#[tauri::command]` functions are registered via
    //      `generate_handler![...]`.
    //   3. The webview window described in tauri.conf.json is created.
    //
    // HINT: when you implement the command bodies in lib.rs, this file stays
    // untouched — that is the whole point of the split.
    app_lib::run();
}
