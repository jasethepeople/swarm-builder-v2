
import { invoke } from '@tauri-apps/api/core'; import { listen } from '@tauri-apps/api/event';
export async function listModels(){ return await invoke<string[]>('whisper_list_models'); }
export async function ensureModel(model:string){ return await invoke<string>('whisper_ensure_model',{model}); }
export async function downloadModel(model:string, onProgress:(p:any)=>void){
  const un = await listen('whisper:download-progress', (e)=>onProgress(e.payload));
  try{ const path = await invoke<string>('whisper_download_model',{model}); return path; } finally{ un(); }
}
export async function transcribe(model:string, audioPath:string){ return await invoke<string>('whisper_transcribe',{model, audioPath}); }
