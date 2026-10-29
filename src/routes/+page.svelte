
<script lang="ts">
import { onMount } from 'svelte';
import { agents, peers, meshStatus, nodeId, transcript, modelProgress, hashLog, initMesh, addAgent, broadcastTask } from '$lib/stores/swarm.svelte.ts';
import { connect } from '$lib/transport/mesh.ts';
import { downloadModel, listModels, ensureModel } from '$lib/voice/whisper.ts';
let peerIdInput=''; let peerAddrInput='tcp://192.168.1.10:5556'; let selectedModel='base'; let models:string[]=[]; let taskInput='run analysis';
onMount(async()=>{ await initMesh(); models = await listModels(); });
async function handleConnect(){ if(peerIdInput && peerAddrInput){ await connect(peerIdInput, peerAddrInput); peers[peerIdInput]=peerAddrInput; } }
async function handleDownload(){ modelProgress={model:selectedModel, pct:0}; await downloadModel(selectedModel, (p)=>{ modelProgress=p; }); modelProgress=null; models=await listModels(); }
</script>
<div class="min-h-screen bg-zinc-950 text-zinc-100 p-4 grid grid-cols-[1fr_380px] gap-4">
  <div class="relative border border-zinc-800 rounded-xl bg-zinc-900/50 overflow-hidden">
    <div class="flex gap-2 p-3 border-b border-zinc-800"><button on:click={addAgent} class="px-3 py-1 bg-white text-black rounded text-xs">+ Add Agent</button><input bind:value={taskInput} class="bg-zinc-800 px-2 py-1 rounded text-xs w-64"/><button on:click={()=>broadcastTask(taskInput)} class="px-3 py-1 bg-zinc-700 rounded text-xs">Broadcast (blake3)</button><span class="ml-auto text-xs text-zinc-500">{meshStatus} | {nodeId}</span></div>
    <div class="relative w-full h-[80vh]">{#each agents as agent}<div class="absolute w-44 p-3 rounded-lg bg-zinc-800 border border-zinc-700 text-xs" style="left:{agent.x}px; top:{agent.y}px">{agent.id}<br/><span class="text-zinc-500">{agent.role} - {agent.status}</span></div>{/each}</div>
  </div>
  <div class="space-y-4">
    <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-900"><h3 class="text-xs font-bold mb-2">MESH NETWORK - ROUTER P2P (ZeroMQ)</h3><div class="text-[10px] text-zinc-500 mb-2">Listening as ROUTER. Each peer is ROUTER->DEALER.</div><input bind:value={peerIdInput} placeholder="peer id" class="w-full bg-zinc-800 rounded px-2 py-1 text-xs mb-1"/><input bind:value={peerAddrInput} class="w-full bg-zinc-800 rounded px-2 py-1 text-xs mb-2"/><button on:click={handleConnect} class="w-full bg-zinc-700 rounded py-1 text-xs">Connect Peer</button><div class="mt-3 text-[11px]">{#each Object.entries(peers) as [id,addr]}<div class="flex justify-between py-1 border-b border-zinc-800"><span>{id}</span><span class="text-zinc-500">{addr}</span></div>{/each}</div></div>
    <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-900"><h3 class="text-xs font-bold mb-2">VOICE COMMAND - whisper.cpp</h3><select bind:value={selectedModel} class="w-full bg-zinc-800 rounded px-2 py-1 text-xs"><option value="base">base (142MB)</option><option value="small">small (466MB)</option><option value="tiny">tiny (75MB)</option></select><div class="mt-2 flex gap-2"><button on:click={handleDownload} class="flex-1 bg-white text-black rounded py-1 text-xs">Download Model</button><span class="text-[10px] text-zinc-500">{models.includes(`ggml-${selectedModel}.bin`)?'✓ cached':'not cached'}</span></div>{#if modelProgress}<div class="mt-2 h-1 bg-zinc-800 rounded"><div class="h-1 bg-white" style="width:{modelProgress.pct}%"></div></div><div class="text-[10px] text-zinc-500">{modelProgress.pct}%</div>{/if}<textarea bind:value={transcript} placeholder="transcript appears here" class="w-full mt-3 bg-zinc-800 rounded p-2 text-xs h-20"></textarea></div>
    <div class="border border-zinc-800 rounded-xl p-3 bg-zinc-900"><h3 class="text-xs font-bold mb-2">INTEGRITY - blake3</h3><div class="space-y-1 max-h-40 overflow-auto text-[10px] font-mono">{#each hashLog as h}<div class="text-zinc-400">{h}</div>{/each}</div></div>
  </div>
</div>
