<script>
  // @ts-nocheck
  import { open } from "@tauri-apps/api/dialog";
  const extensions = ["jpg", "jpeg", "png"];

  let documents = [];
  let errors = [];

  async function getDocuments() {
    console.log("GET!");
    open({
      multiple: true,
      title: 'Select documents',
      filters: [{
        name: 'Image',
        extensions
      }]
    }).then((newImages) => {
      documents = newImages
    })
  }
</script>

<form>
  <div class="group">
    <label for="file">Choose your document to upload</label>
    <button on:click={getDocuments}>Choose Documents</button>
  </div>

  {#if Array.isArray(documents)}
    {#each documents as document}
      <p>{document}</p>
    {/each}
  {:else if documents === null}
    <br/>
  {:else}
    <p>{documents}</p>
  {/if}
  <!-- <button on:click={}>Submit</button> -->
</form>

<style>
</style>
