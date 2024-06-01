<script lang="ts">
  export let language: any;
  import { invoke } from "@tauri-apps/api";
  import { appDataDir } from "@tauri-apps/api/path";
  import { onMount } from "svelte";
  import UAFlag from "../assets/ua.png";
  import CA from "../assets/CA.png";
  import CN from "../assets/CN.png";
  import CZ from "../assets/CZ.png";
  import EU from "../assets/EU.png";
  import US from "../assets/US.png";
  let cr: any;

  onMount(async () => {
    cr = await invoke("get_currency_data", { data_dir: await appDataDir() });
    // Parse the JSON string into an object
    try {
      cr = JSON.parse(cr);
    } catch (error) {
      console.error("Failed to parse JSON:", error);
    }
    // Set default value for currency calculator
    cr.forEach((currency: any) => {
      currency.calc = 1;
    });
  });
</script>

<div class="content">
  <!-- svelte-ignore a11y-missing-attribute -->
  <h1 class="main-text">
    {language && language["7"]} <img src={UAFlag} width="22px" />
  </h1>
  <!-- If cr object exist render currency exchange for 5 selected currencies in filters -->
  {#if cr}
    {#each cr as currency}
      <div class="currency">
        <div class="currency-info">
          <h3 class="cur-name">
            <!-- Conditionally render image based on currency code -->
            {#if currency.cc[0] === "CAD"}
              <!-- svelte-ignore a11y-missing-attribute -->
              <img src={CA} width="22px" />
              {language && language["14"]}
            {:else if currency.cc[0] === "CNY"}
              <!-- svelte-ignore a11y-missing-attribute -->
              <img src={CN} width="22px" />
              {language && language["15"]}
            {:else if currency.cc[0] === "CZK"}
              <!-- svelte-ignore a11y-missing-attribute -->
              <img src={CZ} width="22px" />
              {language && language["16"]}
              <!-- svelte-ignore a11y-missing-attribute -->
            {:else if currency.cc[0] === "EUR"}
              <!-- svelte-ignore a11y-missing-attribute -->
              <img src={EU} width="22px" />
              {language && language["18"]}
            {:else if currency.cc[0] === "USD"}
              <!-- svelte-ignore a11y-missing-attribute -->
              <img src={US} width="22px" />
              {language && language["17"]}
            {/if}
          </h3>
          <h3 class="cur-rate">
            {language && language["9"]}
            {currency.rate[0]}
          </h3>
          <h3 class="cur-val">{language && language["10"]} {currency.cc[0]}</h3>
        </div>

        <div class="currency-calc">
          <h3 class="cur-name">{language && language["11"]}</h3>
          <input
            type="text"
            placeholder="{language && language["12"]}"
            bind:value={currency.calc}
          />
          <!-- The second input field is pre-filled with the result of the multiplication -->
          <input
            type="text"
            value={currency.calc * currency.rate[0]}
            readonly
          />
        </div>
      </div>
    {/each}
  {:else}
    <p>{language && language["8"]}</p>
  {/if}
</div>

<!-- Have a nice day :) -->