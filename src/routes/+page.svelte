<script lang="ts">
    // @ts-nocheck

    import ClaimSearch from "../components/ClaimSearch.svelte";
    import TitleBar from "../components/TitleBar.svelte";
    import WarrantyForm from "../components/WarrantyForm.svelte";
    import Transition from "../components/Transition.svelte";
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";

    enum State {
        CLAIM_FORM = "CLAIM_FORM",
        CLAIM_SEARCH = "CLAIM_SEARCH",
    }

    let state = State.CLAIM_FORM;

    onMount(() => {
        invoke("show_main");
    });
</script>

<svelte:head>
    <link
        rel="stylesheet"
        href="https://fonts.googleapis.com/icon?family=Material+Icons"
    />
    <link
        rel="stylesheet"
        href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"
    />
</svelte:head>

<TitleBar bind:state />
<div class="container">
    {#if state === State.CLAIM_FORM}
        <Transition>
            <WarrantyForm />
        </Transition>
    {:else if state === State.CLAIM_SEARCH}
        <Transition>
            <ClaimSearch />
        </Transition>
    {/if}
</div>

<style>
    .container {
        margin: 0 auto;
        float: unset;
        height: 100%;
        width: 100%;
    }
</style>
