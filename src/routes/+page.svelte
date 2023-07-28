<script lang='ts'>
    // @ts-nocheck
    import * as yup from 'yup';
    import { quintOut } from 'svelte/easing'
    import {tweened} from 'svelte/motion';
    import TitleBar from '../components/TitleBar.svelte'
    import NavButtons from '../components/NavButtons.svelte';
    import Parts from '../components/Parts.svelte';
    import Transition from '../components/Transition.svelte';
    
    const progress = tweened(1, {
        duration: 400,
        easing: quintOut
    });

    $: progress.set((step / 3) * 100)

    const partSchema = yup.object({
        part_number: yup.string().trim().min(1).required(),
        invoice_number: yup.number().required(),
        distributor_number: yup.number().required(),  
    }); 

    const invoiceSchema = yup.object({
        customer_first_name: yup.string().trim().min(3).required(),
        customer_last_name: yup.string().trim().min(3).required(),
        customer_address_1: yup.string().trim().required(),
        customer_city: yup.string().trim().required(),
        customer_zip_code: yup.string().trim().required(),
        customer_phone_number: yup.number().min(1000000000).max(9999999999).required(),
        product_code: yup.string().trim().min(8).max(10).required(),
        serial_number: yup.number().min(10).max(10).required(),
        model_number: yup.string().trim().min(8).required(),
        purchase_date: yup.date().required(),
        miles_traveled: yup.number().min(1).max(999).required(),
        issue_description: yup.string().trim().required(),
        parts: yup.array().of(partSchema),
    });

    let invoice = {
        parts: [],
    }; 
    
    let errors = {}

    async function submitClaim() {
        try {
            await invoiceSchema.validate(invoice, {abortEarly: false});
            alert(JSON.stringify(invoice, null, 2));
            errors = {};
        } catch (error) {
            errors = error.inner.reduce((acc, err) => {
                return { ...acc, [err.path]: err.message };
            }, {});
        }
    }

    let step = 0;
    let prevStep;
    let numSteps = 3;

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
<br/>
<div class="container">
    <progress value="{$progress}" max="100" class="progress"/>
    {#if step === 0}
    <Transition key={step}>
        <div class="form-section">
            <h2>Customer Information</h2>
            <label>First Name: <input bind:value={invoice.customer_first_name} /></label>
            <label>Last Name: <input bind:value={invoice.customer_last_name} /></label>
            <label>Street Address: <input bind:value={invoice.customer_address_1} /></label>
            <label>Zip Code: <input bind:value={invoice.customer_zip_code} /></label>
            <label>Phone: <input bind:value={invoice.customer_phone_number} type="tel"/></label>
        </div>
    </Transition>
    {:else if step === 1}
        <div class="form-section">
            <h2>Appliance Information</h2>
            <label>Product Code: <input bind:value={invoice.product_code} /></label>
            <label>Model Number: <input bind:value={invoice.model_number} /></label>
            <label>Serial Number: <input bind:value={invoice.serial_numbeer} type="number" max="999999999"/></label>
            <label>Purchase Date: <input bind:value={invoice.purchase_date} type="date" /></label>
        </div>
    {:else if step === 2}
        <div class="form-section">
            <h2>Labour Information</h2>
            <label>Miles Traveled: <input bind:value={invoice.miles_traveled} type="number" min="1" /></label>
            <label>Description of Issue: <input bind:value={invoice.issue_description}/></label>
        </div>
    {:else if step === 3}
        <div class="form-section">
                    <Parts {invoice}/>
        </div>
        {#if JSON.stringify(errors) != '{}'}
            {#each Object.entries(errors) as [key, error]}
            <div class="form-error"><span>{key}</span> : <span>{error}</span></div>
            {/each}
        {/if}
        <div class="form-section">
            <button on:click|preventDefault={submitClaim} data-tooltip="Make sure you have everything you need!">Submit</button>
        </div>
    {/if}
        <NavButtons {prev} {next} {step}/>
</div>

<style> 
    .container {
        margin-left: 25px;
    }
    .progress {
        position: fixed;
        top: 40px;
        width: 95%;
        margin: auto;
    }
    button:active {
        transform: scale(0.97);
    }

    .form-section {
        margin: 10px auto;
        width: 95vw;
    }
    h2 {
        text-align: center;
    }
</style>