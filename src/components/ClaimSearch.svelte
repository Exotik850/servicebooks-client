<script lang="ts">
    //@ts-nocheck

    import { invoke } from '@tauri-apps/api';

    let claimNumber = '';
    let claim = null;

    async function getClaim() {
        await invoke("get_claim", {claimNumber: claimNumber})
        .then((respo) => {claim = respo;})
        .catch((error) => {console.log(error)})
    }
</script>

<h2>Claim Search</h2>
<article>
    <label>Claim Number: <input type="search" bind:value={claimNumber}/></label>
    <button on:click={getClaim}></button>

    {#if claim}
        <p>{claim.modelNumber}</p>
    {/if}
</article>

<style>
    h2 {
        text-align: center;
    }
    article {
        width: 95%;
        margin-left: auto;
        margin-right: auto;
    }
</style>