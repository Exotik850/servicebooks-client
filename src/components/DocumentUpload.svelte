<script lang="ts">
  // @ts-nocheck
  import { open } from "@tauri-apps/api/dialog";
  import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  const extensions = ["jpg", "jpeg", "png"];

  let unlisten = null;

  onMount(async () => {
    unlisten = await listen("tauri://file-drop", (e) => {
      console.log(e);
      if (e.payload.length > 1) {
        errors = ["You can only drop one document at a time!"];
        return;
      }

      if (extensions.includes(e.payload[0].split(".").pop())) {
        console.log("File set by drag and drop");
        doc = e.payload[0];
      } else {
        errors = ["You can only upload images at this time!"];
      }
    });
  });

  onDestroy(() => {
    unlisten();
  });

  let doc = null;
  let errors = [];
  let getQb = false;
  let getSb = false;
  let claimNumber = "";
  let description = "";
  $: currentImageView = doc ? convertFileSrc(doc) : "";

  async function getDocuments() {
    errors = [];
    let newImage = await open({
      multiple: false,
      title: "Select documents",
      filters: [
        {
          name: "Documents",
          extensions,
        },
      ],
    }).catch((error) => {
      errors = [error];
    });
    doc = newImage;
  }

  async function submitDocument() {
    invoke("upload_document", {
      document: doc,
      getQb,
      getSb,
      claimNumber,
      description,
    })
      .then((result) => {
        console.log(result);
      })
      .catch((err) => {
        console.error(err);
      });
  }
</script>

{#each errors as error}
  <input type="text" placeholder={error} readonly aria-invalid="true" />
{/each}

<div>
  <form>
    <div class="group">
      <label for="file"
        >Choose your document to upload (You can also drag and drop)</label
      >
      <button on:click={getDocuments}>Choose Document</button>
    </div>

    {#if doc != null}
      <div class="container">
        <img class="image" src={currentImageView} />
        <br />
        <input placeholder="Claim Number" bind:value={claimNumber} />
        <textarea
          rows="2"
          placeholder="Description"
          style="resize: none;"
          bind:value={description}
        />
        <div class="submit-select">
          <span>Send To:</span>
          <fieldset>
            <label for="quickbooks">
              Quickbooks
              <input
                type="checkbox"
                id="quickbooks"
                role="switch"
                bind:checked={getQb}
              />
            </label>
            <label for="servicepower">
              Servicepower
              <input
                type="checkbox"
                id="servicepower"
                role="switch"
                bind:checked={getSb}
              />
            </label>
          </fieldset>
        </div>
        <button
          on:click={submitDocument}
          disabled={claimNumber === "" || description === "" || (!getQb && !getSb)}>Submit</button
        >
      </div>
    {:else}
      <br />
    {/if}
    <br />
    <br />
  </form>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .submit-select {
    display: flex;
    flex-direction: column;
    /* align-items: center; */
    align-items: center;
  }
  .submit-select fieldset {
    justify-content: space-evenly;
    display: flex;
  }
  .image {
    max-width: 75%;
    margin: 0 auto;
  }
</style>
