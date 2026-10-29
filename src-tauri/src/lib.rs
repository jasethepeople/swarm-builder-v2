
mod mesh;
mod whisper_manager;
use mesh::*;
use whisper_manager::*;
#[tauri::command]
fn blake3_hash(input: String) -> String { blake3::hash(input.as_bytes()).to_hex().to_string() }
#[tauri::command]
fn blake3_hash_file(path: String) -> Result<String,String> {
  let data = std::fs::read(&path).map_err(|e| e.to_string())?;
  Ok(blake3::hash(&data).to_hex().to_string())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
      mesh_listen, mesh_connect, mesh_disconnect, mesh_peers, mesh_broadcast, mesh_send,
      blake3_hash, blake3_hash_file,
      whisper_list_models, whisper_ensure_model, whisper_download_model, whisper_transcribe
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri app");
}
