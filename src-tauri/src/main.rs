#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::async_runtime::block_on;
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
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_12.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_13.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_14.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_15.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_16.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_17.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_18.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_19.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_20.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_21.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_22.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_23.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_24.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_25.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_26.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_27.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_28.png").to_vec()));
  icon_vec.push(Icon::Raw(include_bytes!("../assets/images/loading_29.png").to_vec()));

  return icon_vec;
}

fn set_default_icon<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  app.tray_handle().set_icon(tray_icon()).unwrap();
  Ok(())
}

// 系统托盘图标 loading 动画效果采用的是定时器每隔 30 ms 切换图片实现的。
// 现在碰到的问题是在切换回默认的托盘图标定时器没有销毁，还在一直调用。
// 前端有 clearInterval 方法可以销毁定时器，但不知道 rust 是怎么处理的。
async fn set_loading_icon<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
  let ms = 30;
  let icon_vec = tray_icon_loading();

  let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(ms));
  tokio::spawn(async move {
    let mut i = 0;
    loop {
      interval.tick().await;
      app.tray_handle().set_icon(icon_vec[i].clone()).unwrap();
      i = if i >= 29 { 0 } else { i + 1 };
      // force break for test
      // if i >= 29 {
      //   app.tray_handle().set_icon(tray_icon()).unwrap();
      //   break;
      // }
    }
  });
  Ok(())
}

#[tauri::command]
fn change_tray_icon<R: Runtime>(app: AppHandle<R>, icon_type: String) {
  match icon_type.as_str() {
    "default" => {
      set_default_icon(app.clone()).unwrap();
    }
    "loading" => {
      block_on(set_loading_icon(app.clone())).unwrap();
    }
    _ => {
      set_default_icon(app.clone()).unwrap();
    }
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
