<script lang="ts">
  // @ts-nocheck
  import { slide, fly } from "svelte/transition";

  export let invoice;

  function deletePart(index) {
    invoice.parts.splice(index, 1);
    invoice = invoice;
  }

  function addPart() {
    invoice.parts.push({
      part_number: "",
      invoice_number: null,
      distributor_number: null,
      description: null,
      desc_opt: false,
      id: invoice.parts.length,
    });
    invoice = invoice;
  }
</script>

<div class="parts-used">
  {#each invoice.parts as part, index (index)}
    <div class:bottom={index != invoice.parts.length - 1} transition:slide>
      <div class="part" style="justify-content: left;">
        <span>Part {index + 1}</span>|
        <label data-tooltip="Only used when item not in QB" style="border-bottom: none;" for="description_{part.id}">
          Description
          <input
            type="checkbox"
            role="switch"
            id="description_{part.id}"
            bind:checked={part.desc_opt}
          />
        </label>
      </div>
      <div class="part">
        <label>Part Number: <input bind:value={part.part_number} /></label>
        <label
          >Invoice Number: <input
            bind:value={part.invoice_number}
            required
          /></label
        >
        <label
          >Distributor:
          <select bind:value={part.distributor_number}>
            <option value="0000100675">Marcone</option>
            <option value="0000100777">Reliable Parts</option>
            <option value="000114927">Encompass</option>
          </select>
        </label>
        <button class="delete" on:click|preventDefault={deletePart(index)}>
          <i class="material-icons">close</i>
        </button>
      </div>
      {#if part.desc_opt}
        <input bind:value={part.description} placeholder="Part Description" transition:slide/>
      {/if}
    </div>
  {/each}
  <button on:click|preventDefault={addPart} class="add secondary"
    >Add Part</button
  >
</div>

<style>
  .part {
    display: flex;
    justify-content:space-around;
    gap: 5px;
  }
  .bottom {
    border-bottom: 2px;
    border-color: var(--primary);
  }

  .delete {
    background: rgb(126, 21, 21);
    color: rgb(219, 219, 219);
    border: rgb(78, 13, 13);
    padding: 0;
    margin-top: auto;
    margin-bottom: auto;
    left: 10px;
    transition: 0.3s ease;
    justify-content: center;
    align-items: center;
    display: flex;
    width: 45px;
    height: 40px;
    border-radius: 50%;
    box-shadow: none;
  }

  .delete:hover {
    background: darkred;
    transform: scale(1.1);
  }

  .delete:active {
    background: darkred;
    transform: scale(0.9);
  }

  .add {
    transition: 0.3s ease;
  }
</style>
