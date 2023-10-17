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
        filePath = e.payload[0];
      } else {
        errors = ["You can only upload images at this time!"];
      }
    });
  });

  onDestroy(() => {
    unlisten();
  });

  let errors = [];
  let success = null;
  let sent = null;
  let filePath = null;
  let uploadQb = false;
  let uploadSp = false;
  let claimNumber = "";
  let imageDescription = "";
  $: currentImageView = filePath ? convertFileSrc(filePath) : "";

  async function getDocuments() {
    errors = [];
    let newImage = await open({
      multiple: false,
      title: "Select documents",
      filters: [
        {
          name: "Documents (Only images as of now)",
          extensions,
        },
      ],
    }).catch((error) => {
      errors = [error];
    });
    filePath = newImage;
  }

  async function submitDocument() {
    invoke("upload_document", {
      filePath,
      uploadQb,
      uploadSp,
      claimNumber,
      imageDescription,
    })
      .then((result) => {
        console.log(result);
        sent = "";
        if (uploadQb) sent += "Quickbooks";
        if (uploadQb && uploadSp) sent += " and ";
        if (uploadSp) sent += "ServicePower";
        success = filePath;
        filePath = null;
        uploadQb = false;
        uploadSp = false;
        claimNumber = "";
        imageDescription = "";
      })
      .catch((err) => {
        errors = [err];
        sent = null;
        success = null;
      });
  }
</script>

{#if success !== null}
  <article style="background-color: green;">
    <h2>Success!</h2>
    <p>Uploaded {success} to {sent}</p>
  </article>
{/if}

{#each errors as error}
  <article class="error"><p>{error}</p></article>
{/each}

<div>
  <form>
    <div class="group">
      <label for="file"
        >Choose your document to upload (You can also drag and drop)</label
      >
      <button on:click={getDocuments}>Choose Document</button>
    </div>

    {#if filePath != null}
      <div class="container">
        <img class="image" src={currentImageView} alt="Selected document" />
        <br />
        <input placeholder="Claim Number" bind:value={claimNumber} />
        <textarea
          rows="2"
          placeholder="Description"
          style="resize: none;"
          bind:value={imageDescription}
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
                bind:checked={uploadQb}
              />
            </label>
            <label for="servicepower">
              Servicepower
              <input
                type="checkbox"
                id="servicepower"
                role="switch"
                bind:checked={uploadSp}
              />
            </label>
          </fieldset>
        </div>
        <button
          on:click={submitDocument}
          disabled={claimNumber === "" ||
            imageDescription === "" ||
            (!uploadQb && !uploadSp)}>Submit</button
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
  .error {
    border: 1px solid red;
    padding: 1em;
    margin: 1em 0;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
