<script lang="ts">
    //@ts-nocheck

    import { invoke } from '@tauri-apps/api';
    import Transition from './Transition.svelte';

    let claimNumber = '';
    let claim = null;
    let error = null;

    async function getClaim() {
        error = "";
        if (claimNumber == '') {
            claim = null;
            return
        }
        await invoke("get_claim", {claimNumber: claimNumber})
        .then((respo) => {claim = respo;})
        .catch((err) => {
            console.error(err);
            error = err;
        })
    }

    function displayObject(obj) {
    if (obj && typeof obj === 'object') {
      return Object.entries(obj).map(([key, value]) => {
        if (typeof value === 'object') {
          return `<div class="nested">
            <p><strong>${key}:</strong></p>
            ${displayObject(value)}  
          </div>`;
        } else {
          return `<p><strong>${key}:</strong> ${value}</p>`;
        }
      }).join(''); 
    }
  }

</script>

<h2>Claim Search</h2>
<article>
    <form>
        <label>Claim Number: <input type="search" bind:value={claimNumber}/></label>
        <button type="submit" on:click|preventDefault={getClaim}>Search</button>
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

{#if claim}
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
    .claim {

    }
</style>