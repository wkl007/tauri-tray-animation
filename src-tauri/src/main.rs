#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{Runtime, AppHandle, Icon, SystemTray};

fn tray_icon() -> Icon {
  return Icon::Raw(include_bytes!("../assets/images/default.png").to_vec());
}

fn tray_icon_loading() -> Vec<Icon> {
  let mut icon_vec: Vec<Icon> = Vec::new();
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_0.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_1.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_2.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_3.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_4.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_5.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_6.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_7.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_8.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_9.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_10.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_11.png").to_vec()));

  return icon_vec;
}

#[tauri::command]
fn change_tray_icon<R: Runtime>(app: AppHandle<R>, icon_type: String, icon_index: usize) {
  if icon_type == "default" {
    app.tray_handle().set_icon(tray_icon()).unwrap();
  } else if icon_type == "loading" {
    let icon_vec = tray_icon_loading();
    app.tray_handle().set_icon(icon_vec[icon_index].clone()).unwrap();
  }
}

fn tray() -> SystemTray {
  let tray = SystemTray::new();
  return tray;
}

fn main() {
  tauri::Builder::default()
    .system_tray(tray())
    .invoke_handler(tauri::generate_handler![
      change_tray_icon
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
