use tauri::{
    Manager, WindowEvent,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let show = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let exit = MenuItem::with_id(app, "exit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &exit])?;
            let _tray = TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.restore_state(StateFlags::POSITION | StateFlags::SIZE);
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "exit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app_handle = window.app_handle();
                app_handle
                    .save_window_state(StateFlags::POSITION | StateFlags::SIZE)
                    .ok();
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.restore_state(StateFlags::POSITION | StateFlags::SIZE);
                let _ = window.show();
                let _ = window.set_always_on_bottom(true);
                let _ = window.set_focus();
            }
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
