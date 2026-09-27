use tauri::Manager;

fn parse_harness_url(value: &str) -> Result<tauri::Url, String> {
  let value = value.trim();
  if !(value.starts_with("http://") || value.starts_with("https://")) {
    return Err("请输入以 http:// 或 https:// 开头的完整地址。".into());
  }
  let url = tauri::Url::parse(value).map_err(|_| "地址格式不正确，请重新复制完整地址。")?;
  if url.host_str().is_none() || !url.username().is_empty() || url.password().is_some() {
    return Err("请输入有效的 Web 地址，认证 token 应放在 ?token= 参数中。".into());
  }
  Ok(url)
}

// WebView creation runs asynchronously to avoid a Windows WebView2 deadlock.
#[tauri::command]
async fn connect_harness(
  app: tauri::AppHandle,
  window: tauri::WebviewWindow,
  address: String,
) -> Result<(), String> {
  if window.label() != "main" {
    return Err("请从启动窗口连接。".into());
  }
  let url = parse_harness_url(&address)?;
  if app.get_webview_window("harness").is_some() {
    return Err("请先关闭当前 Harness 窗口，再连接新地址。".into());
  }
  tauri::WebviewWindowBuilder::new(&app, "harness", tauri::WebviewUrl::External(url))
    .title("DeepSeek Harness · 关闭此窗口可更换地址")
    .inner_size(1500.0, 1000.0)
    .build()
    .map_err(|_| "无法打开 Harness 窗口，请重试。")?;
  // Keep the local launcher alive so closing Harness allows another connection.
  let _ = window.hide();
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![connect_harness])
    .on_window_event(|window, event| {
      if window.label() == "harness" && matches!(event, tauri::WindowEvent::Destroyed) {
        if let Some(launcher) = window.app_handle().get_webview_window("main") {
          let _ = launcher.show();
          let _ = launcher.set_focus();
        }
      }
    })
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
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
  use super::parse_harness_url;

  #[test]
  fn preserves_token_and_custom_port() {
    let address = "http://127.0.0.1:4090/?token=test%2Btoken&other=1";
    assert_eq!(parse_harness_url(&format!("  {address}\n")).unwrap().as_str(), address);
  }

  #[test]
  fn rejects_invalid_addresses_and_non_web_schemes() {
    for address in ["", "127.0.0.1:3080", "javascript:alert(1)", "file:///tmp/a", "http://", "http://user:secret@localhost/"] {
      assert!(parse_harness_url(address).is_err());
    }
  }
}
