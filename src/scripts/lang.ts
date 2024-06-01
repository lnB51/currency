import { invoke } from "@tauri-apps/api";
import EN from "../config/language/en.json";
import UK from "../config/language/uk.json";
import { appDataDir } from "@tauri-apps/api/path";

async function change_language(lang: string)  {
    invoke('modify_config', { id: "language", value:lang, data_dir: `${await appDataDir()}`})
    if (lang === "uk") {
        return UK;
    } else{
        return EN;
    }
}

export {change_language}