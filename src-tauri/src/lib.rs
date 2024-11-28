use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .setup(|app| {
            #[cfg(desktop)]
            {
                let alt_n_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyN);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_, shortcut, event| {
                            if tauri::is_dev() {
                                println!("{:#?}", shortcut);

                                if shortcut == &alt_n_shortcut {
                                    match event.state() {
                                        ShortcutState::Pressed => {
                                            println!("Alt-N Pressed");
                                        }
                                        ShortcutState::Released => {
                                            println!("Alt-N Released");
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
