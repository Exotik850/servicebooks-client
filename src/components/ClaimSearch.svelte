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
            {#each Object.entries(claim) as [key, value]}
                <p><strong>{key}:</strong> {value}</p>
            {/each}
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