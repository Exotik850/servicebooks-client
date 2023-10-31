<script lang="ts">
  //@ts-nocheck

  import { invoke } from "@tauri-apps/api";
  import { displayObject } from "./util";
  import Transition from "./Transition.svelte";

  let claimNumber = "";
  let claim = null;
  let error = null;
  let loading = false;
  let getQb = false;
  let getSb = false;

  async function getClaim() {
    error = "";
    loading = true;
    if (claimNumber == "") {
      claim = null;
      return;
    }
    await invoke("get_claim", {
      claimNumber: claimNumber,
      getQb: getQb,
      getSb: getSb,
    })
      .then((respo) => {
        claim = respo;
      })
      .catch((err) => {
        console.error(err);
        error = err;
      })
      .finally(() => {
        loading = false;
      });
  }
</script>

<h2>Claim Search (Will look better soon!)</h2>
<article>
  <form>
    <label>Claim Number: <input type="search" bind:value={claimNumber} /></label
    >
    Get from:
    <fieldset>
      <label for="quickbooks">
        Quickbooks
        <input type="checkbox" id="quickbooks" bind:checked={getQb} />
      </label>
      <label for="servicepower">
        Servicepower
        <input type="checkbox" id="servicepower" bind:checked={getSb} />
      </label>
    </fieldset>
    <button
      type="submit"
      disabled={(!getQb && !getSb) || !claimNumber}
      on:click|preventDefault={getClaim}>Search</button
    >
  </form>
</article>

{#if error}
  <div class="error">
    <Transition>
      <article>
        <p>{error}</p>
      </article>
    </Transition>
  </div>
{/if}

{#if loading}
  <article aria-busy="true" class="secondary" />
{:else if claim}
  <div class="claim">
    <Transition>
      <article>
        {@html displayObject(claim)}
      </article>
    </Transition>
  </div>
{/if}

<style>
  h2 {
    text-align: center;
  }
  article {
    width: 95%;
    margin-left: auto;
    margin-right: auto;
  }
  .error article {
    background-color: #af0606;
  }
  .error p {
    color: antiquewhite;
    text-align: center;
    margin: 0 auto;
  }
  fieldset {
    display: flex;
    justify-content: space-evenly;
  }
</style>
