<script lang='ts'>
    // @ts-nocheck
    import { fade } from "svelte/transition";

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

<h2>Parts Used</h2>
    <div class="parts-used">
        {#each invoice.parts as part, index (index)}
            <div class="part" class:bottom={index != invoice.parts.length - 1} transition:fade>
                <label>Part {index + 1} Part Number: <input bind:value={part.part_number} /></label>
                <label>Invoice Number: <input bind:value={part.invoice_number} type="number" min="0" /></label>
                <label>Distributor: <input bind:value={part.distrubutor_number} type="number" min="0" /></label>
                <button class='delete' on:click|preventDefault={deletePart(index)}>x</button>
            </div>
        {/each}
    <button on:click|preventDefault={addPart} class="add">Add Part</button>
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
        border: none;
        padding: 5px 7px;
        font-size: 1rem;
        margin:auto;
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