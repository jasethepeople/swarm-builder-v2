
import { invoke } from '@tauri-apps/api/core';
export const listen = (port:number, nodeId:string)=>invoke('mesh_listen',{port, nodeId});
export const connect = (peerId:string, address:string)=>invoke('mesh_connect',{peerId, address});
export const peers = ()=>invoke('mesh_peers');
export const broadcast = (payload:string)=>invoke('mesh_broadcast',{payload});
export const send = (peerId:string, payload:string)=>invoke('mesh_send',{peerId, payload});
