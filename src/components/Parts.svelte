<script lang='ts'>
    // @ts-nocheck
    import { slide } from "svelte/transition";

    export let invoice;

    function deletePart(index) {
        invoice.parts.splice(index, 1);
        invoice = invoice
    }

    function addPart() {
        invoice.parts.push({part_number: '', invoice_number: '', distributor_number: ''});
        invoice = invoice
    }
</script>

<div class="parts-used">
    {#each invoice.parts as part, index (index)}
    <div class:bottom={index != invoice.parts.length - 1} transition:fade>
        <span>Part {index + 1}</span>
        <div class="part">
            <label>Part Number: <input bind:value={part.part_number} /></label>
            <label>Invoice Number: <input bind:value={part.invoice_number} type="number" min="0" /></label>
            <label>Distributor: 
                <select bind:value={part.distributor_number}>
                    <!-- TODO: ADD DISTRIBUTOR NUMBERS -->
                    <option value="" selected>Marcone</option>
                    <option value="">Reliable Parts</option>
                    <option value="">Encompass</option>
                </select>
            </label>
            <button class='delete' on:click|preventDefault={deletePart(index)}>x</button>
        </div>
    </div>
    {/each}
    <button on:click|preventDefault={addPart} class="add secondary">Add Part</button>
</div>
<style>
    .part {
        display: flex;
        align-items: center;
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
        font-size: 1rem;
        margin-top:auto;
        margin-bottom:auto;
        left: 10px;
        transition: 0.3s ease;  
        width: 45px;
        height: 40px;
        border-radius: 50%;
    }

    .delete:hover {
        background: darkred;
        transform: scale(1.1);
    }

    .add {
        transition: 0.3s ease;
    }
</style>