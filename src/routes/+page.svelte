<script lang='ts'>
    // @ts-nocheck
    import TitleBar from '../components/TitleBar.svelte'
    import { enhance } from '$app/forms';
    import type { ActionData } from './$types';
    import { invoiceSchema } from '$lib/invoiceSchema';
    import { quintOut } from 'svelte/easing'
    import {tweened} from 'svelte/motion';
    
    let step = 0;
    let prevStep;
    let numSteps = 3;

    const progress = tweened(1, {
    duration: 400,
    easing: quintOut
  });
  $: progress.set((step / 3) * 100)

    export let form: ActionData;
    $: console.log(form)

    let invoice = {
        customer: {},
        appliance: {},
        labour: {},
        parts: [],
    };   

    function exists(data){
        data !== null && data !== undefined
    }

    function next() {
        if (step < numSteps) {
            prevStep = step;
            step++;
        }
    }

    function prev() {
        if (step > 0) {
            prevStep = step;
            step--;
        }
    }
</script>
<TitleBar/>
<div>
    <progress value="{$progress}" max="100" class="form-section"/>
    <form use:enhance method="POST">
    {#if step === 0}
        <div class="form-section">
            <h2>Customer Information</h2>
            <label>Name: <input bind:value={invoice.customer.name} /></label>
            <label>Email: <input bind:value={invoice.customer.email} type="email" /></label>
            <label>Phone: <input bind:value={invoice.customer.phone} /></label>
        </div>
    {:else if step === 1}
        <div class="form-section">
            <h2>Appliance Information</h2>
            <label>Model: <input bind:value={invoice.appliance.model} /></label>
            <label>Serial Number: <input bind:value={invoice.appliance.serialNumber} type="number" max="999999999"/></label>
            <label>Purchase Date: <input bind:value={invoice.appliance.purchaseDate} type="date" /></label>
        </div>
    {:else if step === 2}
        <div class="form-section">
            <h2>Labour Information</h2>
            <label>Miles Traveled: <input bind:value={invoice.labour.hours} type="number" min="0" /></label>
            <label>Description of Issue: <input bind:value={invoice.labour.rate} type="number" min="0" /></label>
        </div>
    {:else if step === 3}
        <div class="form-section">
            <h2>Parts Used</h2>
            <div class="parts-used">
                {#each invoice.parts as part, index (index)}
                    <div class="part">
                        <label>Part {index + 1} Name: <input bind:value={part.name} /></label>
                        <label>Part {index + 1} Invoice Number: <input bind:value={part.invoice_number} type="number" min="0" /></label>
                        <label>Part {index + 1} Distributor: <input bind:value={part.distrubutor_number} type="number" min="0" /></label>
                    </div>
                {/each}
                <button on:click|preventDefault={() => {invoice.parts.push({name: '', cost: 0}); invoice = invoice}}>Add Part</button>
            </div>
        </div>
        {/if}
        
    {#if step == 3}
    {#if form?.packet.error}
        <div class="form-section" aria-invalid="true">
        {#each form.packet.errors as error}
            <li>{error.field} : {error.message}</li>
        {/each}
        </div>
    {/if}
    <div class="form-section form-submit">
        <button type="submit" disabled={step != 3 && !exists(form?.packet.error)}>Submit</button>
    </div>
    {/if}
    
    <div class="form-navigation" style={"position: relative;"*(step != numSteps) || "position:absolute"}}>
        <button on:click|preventDefault={prev} disabled={step === 0}>Previous</button>
        <button on:click|preventDefault={next} disabled={step === 3}>Next</button>
    </div>
    </form>
</div>

<style> 
    .form-navigation {
        display: flex;
        gap: 20px;
        margin: 10px 10px auto;
        width: 95vw;
        bottom: 0%;
    }
    .form-submit {
        /* position: fixed; */
        bottom: 70px;
    }
    .form-section {
        margin: 10px 10px auto;
        width: 95vw;
    }
</style>