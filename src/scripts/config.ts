import { invoke } from "@tauri-apps/api";
import { appDataDir } from "@tauri-apps/api/path";

async function change_output(out_format: string)  {
    invoke('modify_config', { id: "output", value:out_format, data_dir: `${await appDataDir()}`});
    invoke("fetch_and_save_currency_rates", {data_dir: await appDataDir(), manual: "true"});
    
}

async function change_frq(event: Event)  {
    const target = event.target as HTMLInputElement;
    const frq = target.value;
    invoke('modify_config', { id: "frequency", value:frq, data_dir: `${await appDataDir()}`});
}

async function open_data_dir()  {
    invoke('open_data_dir', { data_dir: `${await appDataDir()}`});
}

async function open_exchange_rate()  {
    invoke('open_exchange_rate', { data_dir: `${await appDataDir()}`});
}

export {open_exchange_rate, change_output, change_frq, open_data_dir}