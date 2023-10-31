<script lang="ts">
  // @ts-nocheck
  import { open } from "@tauri-apps/api/dialog";
  import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import ErrorPanel from "./ErrorPanel.svelte";
  const extensions = ["jpg", "jpeg", "png"];

  let unlisten = null;

  onMount(async () => {
    unlisten = await listen("tauri://file-drop", (e) => {
      console.log(e);
      if (e.payload.length > 1) {
        errorPanel.setError("You can only drop one document at a time!");
        return;
      }

      let ext = e.payload[0].split(".").pop();

      if (extensions.includes(ext)) {
        console.log("File set by drag and drop");
        filePath = e.payload[0];
      } else {
        errorPanel.setError("You can only upload images at this time!");
      }
    });
  });

  onDestroy(() => {
    unlisten();
  });

  // let errors = [];
  // let success = null;
  // let sent = null;
  let errorPanel;
  let filePath = null;
  let uploadQb = false;
  let uploadSp = false;
  let isSalesReceipt = false;
  let claimNumber = "";
  let imageDescription = "";
  $: currentImageView = filePath ? convertFileSrc(filePath) : "";

  async function getDocuments() {
    errorPanel.reset();
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
      errorPanel.setError(error);
    });
    filePath = newImage;
  }

  function submitDocumentPromise() {
    return new Promise((resolve, reject) => {
      invoke("upload_document", {
        filePath,
        uploadQb,
        uploadSp,
        isSalesReceipt,
        claimNumber,
        imageDescription,
      })
        .then((result) => {
          console.log(result);
          let sent = "";
          if (uploadQb) sent += "Quickbooks";
          if (uploadQb && uploadSp) sent += " and ";
          if (uploadSp) sent += "ServicePower";
          success = filePath;
          filePath = null;
          uploadQb = false;
          uploadSp = false;
          claimNumber = "";
          imageDescription = "";
          resolve("Successfully uploaded " + filePath + " to " + sent);
        })
        .catch((err) => reject(err));
    });
  }
</script>

<div>
  <form>
    <div class="group">
      <label for="file"
        >Choose your document to upload (You can also drag and drop)</label
      >
      <button on:click={getDocuments}>Choose Document</button>
    </div>
    <ErrorPanel func={submitDocumentPromise} bind:panel={errorPanel} />
    {#if filePath !== null}
      <div class="container">
        <img class="image" src={currentImageView} alt="Selected document" />
        <br />
        <label for="sales-receipt"
          >Sales Receipt <input
            id="sales-receipt"
            type="checkbox"
            role="switch"
            bind:checked={isSalesReceipt}
          /></label
        >
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
            <label
              for="servicepower"
              data-tooltip="Disabled for now"
              style="border-bottom: none;"
            >
              Servicepower
              <input
                type="checkbox"
                id="servicepower"
                role="switch"
                bind:checked={uploadSp}
                disabled="true"
              />
            </label>
          </fieldset>
        </div>
        <button
          on:click={errorPanel.activate}
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
</style>
