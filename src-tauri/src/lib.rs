use tauri::Manager;

mod hardware;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      hardware::cpu::get_cpu_info,
      hardware::cpu::get_cpu_benchmark,
      hardware::gpu::get_gpu_info,
      hardware::motherboard::get_motherboard_info,
      hardware::memory::get_memory_info,
      hardware::memory::get_physical_memory_info,
      hardware::disks::get_disks_info,
      hardware::disks::get_physical_disks_info,
      hardware::network::get_network_info,
      hardware::network::get_network_adapters_info,
      hardware::system::get_system_info,
    ])
    .setup(|app| {
      #[allow(unused_variables)]
      let window = app.get_webview_window("main").unwrap();

      #[cfg(target_os = "macos")]
      window_vibrancy::apply_vibrancy(&window, window_vibrancy::NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

      #[cfg(target_os = "windows")]
      window_vibrancy::apply_blur(&window, Some((18, 18, 18, 150)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");


      /*#[cfg(target_os = "linux")] {
        // No native blur from window-vibrancy on Linux
        let _ = window.eval("document.body.classList.add('linux-fallback');");
      }*/

      app.handle().plugin(tauri_plugin_process::init())?;
      app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;

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
