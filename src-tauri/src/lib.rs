use tauri::{
  menu::{Menu, MenuItem},
  tray::TrayIconBuilder,
};

pub mod tasks;
use tasks::tasks::get_tasks;

    
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("📥 running back end");

    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![get_tasks])
    .setup(|app| {
        let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
        let menu = Menu::with_items(app, &[&quit_i])?;

        let _tray = TrayIconBuilder::new()
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .show_menu_on_left_click(true)
            .on_menu_event(|app, event| match event.id.as_ref() {
                "quit" => {
                    println!("quit menu item was clicked");
                    app.exit(0);
                }
                _ => {
                    println!("menu item {:?} not handled", event.id);
                }
            })
            .build(app)?;

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


// .on_tray_event(|app, event| match event {
//     SystemTrayEvent::LeftClick { .. } => {
//         let window = app.get_window("main").unwrap();
//         window.show().unwrap();
//         window.set_focus().unwrap();
//     }
//     SystemTrayEvent::MenuItemClick { id, .. } => {
//         match id.as_str() {
//             "show" => {
//                 let window = app.get_window("main").unwrap();
//                 window.show().unwrap();
//                 window.set_focus().unwrap();
//             }
//             "quit" => std::process::exit(0),
//             _ => {}
//         }
//     }
//     _ => {}
// })
