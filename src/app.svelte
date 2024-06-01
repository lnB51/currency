<script lang="ts">
  import Currency from "./lib/currency.svelte";
  import Icon from "./assets/icon.png";
  import UKIcon from "./assets/uk.png";
  import UAIcon from "./assets/ua.png";
  import Settings from "./assets/settings.png";
  import { change_language } from "./scripts/lang";
  import { onMount } from "svelte";
  import { appDataDir } from "@tauri-apps/api/path";
  import { invoke } from "@tauri-apps/api";
  import { open_log } from "./scripts/logs";
  import {
    open_exchange_rate,
    change_output,
    change_frq,
    open_data_dir,
  } from "./scripts/config";

  let language: any;
  let onstart = true;

  async function lang_select(lang: string) {
    language = await change_language(lang);

    // Remove the 'lang-icon-selected' class from all icons
    document.querySelectorAll(".lang-icon").forEach((icon) => {
      icon.classList.remove("lang-icon-selected");
    });

    // Add the 'lang-icon-selected' class to the pressed icon
    document.getElementById(`lang-${lang}`).classList.add("lang-icon-selected");
  }

  async function out_format_select(out: string, on_start: boolean) {
    // Remove the 'select-output-option-selected' class from all params
    document.querySelectorAll(".select-output-option").forEach((icon) => {
      icon.classList.remove("select-output-option-selected");
    });

    // Add the 'lang-icon-selected' class to the pressed label
    document
      .getElementById(`out-${out.toLowerCase()}`)
      .classList.add("select-output-option-selected");

    if (on_start) {
      onstart = false;
    } else {
      change_output(out);
    }
  }

  // Initialize settings with default value
  onMount(async () => {
    const dataDir = await appDataDir();
    const defaultLang: any = await invoke("read_config_value", {
      key: "language",
      data_dir: dataDir,
    });
    const defaultForamt: any = await invoke("read_config_value", {
      key: "output",
      data_dir: dataDir,
    });
    const defaultFrq: any = await invoke("read_config_value", {
      key: "frequency",
      data_dir: dataDir,
    });
    document.getElementById("update-fre").value = defaultFrq;
    out_format_select(defaultForamt, onstart);
    lang_select(defaultLang);
  });
</script>

<main>
  <header>
    <div id="app-info">
      <img src={Icon} alt="icon" id="app-icon" />
      <!-- Check if language object is available -->
      {language && language["0"]}
    </div>
    <div class="settings-icons">
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <img
        src={UKIcon}
        alt="lang"
        id="lang-en"
        class="lang-icon"
        on:click={() => lang_select("en")}
      />
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <img
        src={UAIcon}
        alt="lang"
        id="lang-uk"
        class="lang-icon"
        on:click={() => lang_select("uk")}
      />
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <!-- svelte-ignore a11y-invalid-attribute -->
      <a href="#" class="settings-icon"
        ><img src={Settings} alt="settings" id="settings-icon" />
        <div class="settings-menu">
          <label for="update-fre">{language && language["1"]}</label>
          <input
            type="number"
            id="update-fre"
            name="update-fre"
            min="1"
            max="4"
            on:change={change_frq}
          />
          <label for="update-fre">{language && language["2"]}</label>
          <hr />
          <label for="">{language && language["3"]}</label>
          <div class="select-output">
            <!-- svelte-ignore a11y-missing-content -->
            <a
              href="#"
              id="out-json"
              class="select-output-option select-output-option-selected"
              on:click={() => out_format_select("JSON", false)}>JSON</a
            >
            <a
              href="#"
              id="out-xml"
              class="select-output-option"
              on:click={() => out_format_select("XML", false)}>XML</a
            >
          </div>
          <hr />
          <a
            href="#"
            on:click={() => open_data_dir()}
            class="settings-menu-param">{language && language["4"]}</a
          >
          <hr />
          <a href="#" on:click={() => open_log()} class="settings-menu-param"
            >{language && language["5"]}</a
          >
          <hr />
          <a
            href="#"
            on:click={() => open_exchange_rate()}
            class="settings-menu-param">{language && language["6"]}</a
          >
          <hr />
          <a
            href="#"
            on:click={async () => {
              invoke("fetch_and_save_currency_rates", {
                data_dir: await appDataDir(),
                manual: "true",
              });
            }}
            class="settings-menu-param"
            style="color:#D32F2F;"
            >{language && language["13"]}</a
          >
        </div>
      </a>
    </div>
  </header>
  <Currency {language} />
</main>

<!-- Good luck xD -->