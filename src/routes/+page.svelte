<script lang="ts">
  // @ts-nocheck

  import ClaimSearch from "../components/ClaimSearch.svelte";
  import DocumentUpload from "../components/DocumentUpload.svelte";
  import TitleBar from "../components/TitleBar.svelte";
  import WarrantyForm from "../components/WarrantyForm/WarrantyForm.svelte";
  import Transition from "../components/Transition.svelte";
  
  enum State {
    CLAIM_FORM = "CLAIM_FORM",
    DOCUMENT_UPLOAD = "DOCUMENT_UPLOAD",
    CLAIM_SEARCH = "CLAIM_SEARCH",
  }

  let state = State.CLAIM_FORM;
</script>

<svelte:head>
  <link rel="stylesheet" href="/material_icons.css" />
  <link rel="stylesheet" href="/pico.min.css" />
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
  {:else if state === State.DOCUMENT_UPLOAD}
    <Transition>
      <DocumentUpload />
      <!-- <p>Working on it!</p> -->
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
