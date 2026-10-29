
use std::path::PathBuf;
use tauri::{Emitter, Window};
use futures_util::StreamExt;
use std::io::Write;
pub fn model_dir() -> PathBuf { dirs::data_dir().unwrap_or_else(|| PathBuf::from(".")).join("swarm-builder").join("models") }
pub fn model_path(name: &str) -> PathBuf { model_dir().join(format!("ggml-{}.bin", name)) }
#[tauri::command]
pub fn whisper_list_models() -> Vec<String> {
  let dir = model_dir();
  if !dir.exists() { return vec![]; }
  std::fs::read_dir(dir).map(|r| r.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect()).unwrap_or_default()
}
#[tauri::command]
pub fn whisper_ensure_model(model: String) -> Result<String,String> {
  let p = model_path(&model);
  if p.exists() { Ok(p.to_string_lossy().to_string()) } else { Err(format!("Model {} not cached. Call whisper_download_model", model)) }
}
#[tauri::command]
pub async fn whisper_download_model(window: Window, model: String) -> Result<String,String> {
  let url = format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin", model);
  let dest = model_path(&model);
  std::fs::create_dir_all(model_dir()).map_err(|e| e.to_string())?;
  if dest.exists() { return Ok(dest.to_string_lossy().to_string()); }
  let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
  let total = resp.content_length().unwrap_or(0);
  let mut file = std::fs::File::create(&dest).map_err(|e| e.to_string())?;
  let mut downloaded: u64 = 0;
  let mut stream = resp.bytes_stream();
  while let Some(chunk) = stream.next().await {
    let chunk = chunk.map_err(|e| e.to_string())?;
    file.write_all(&chunk).map_err(|e| e.to_string())?;
    downloaded += chunk.len() as u64;
    let pct = if total>0 { (downloaded as f64 / total as f64 * 100.0) as u8 } else { 0 };
    let _ = window.emit("whisper:download-progress", serde_json::json!({"model":model,"pct":pct,"downloaded":downloaded,"total":total}));
  }
  let bytes = std::fs::read(&dest).map_err(|e| e.to_string())?;
  let hash = blake3::hash(&bytes).to_hex().to_string();
  let _ = window.emit("whisper:download-complete", serde_json::json!({"model":model,"path":dest,"blake3":hash}));
  Ok(dest.to_string_lossy().to_string())
}
#[tauri::command]
pub fn whisper_transcribe(model: String, audio_path: String) -> Result<String,String> {
  let mp = model_path(&model);
  if !mp.exists() { return Err("model not downloaded".into()); }
  // placeholder - hook whisper-rs here
  let fake = format!("[transcribed with {} from {}]", model, audio_path);
  Ok(fake)
}
