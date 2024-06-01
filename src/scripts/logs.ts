import { invoke } from "@tauri-apps/api";
import { appDataDir } from "@tauri-apps/api/path";

async function log(level: string, msg: string)  {
    invoke('log', { level: level, msg: msg, data_dir: `${await appDataDir()}`})
}

async function open_log()  {
    invoke('open_log', { data_dir: `${await appDataDir()}`})
}

export {log, open_log}