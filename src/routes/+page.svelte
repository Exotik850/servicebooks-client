<script lang='ts'>
    // @ts-nocheck
    import TitleBar from '../components/TitleBar.svelte'
    import { invoke } from '@tauri-apps/api'
    import { superForm, superValidate } from 'sveltekit-superforms/client'
    import { onMount } from 'svelte';
    import { z } from 'zod'

    // claim object
    let form;
  
    // submit handler
    async function submitClaim() {
      console.log("Submit?");
      await invoke('submit_claim', { claim });
    }

    onMount(async () => {
        const invoiceSchema = z.object({
            customer_first_name: z.string().minLength(1),
            customer_last_name: z.string().minLength(1),
            customer_address_1: z.string().minLength(1),
            product_code: z.string().minLength(8),
            serial_number: z.number().minValue(1000000000).maxValue(9999999999),
            model_number: z.string().minLength(5),
            date_of_purchase: z.date(),
            description_of_issue: z.string().minLength(10)
        });

        form = await superValidate(invoiceSchema)
    });

</script>
  
<TitleBar />
<br/>
<article>
    <form on:submit|preventDefault={submitClaim}>

        <label for="customer_first_name">Customer First Name</label>
        <input type="text" name="customer_first_name" bind:value={$form.customer_first_name} />

        <label for="customer_last_name">Customer Last Name</label>
        <input type="text" name="customer_last_name" bind:value={$form.customer_last_name} />

        <label for="product_code">Product Code</label>
        <input type="text" name="product_code" bind:value={$form.product_code} />

        <button type="submit">Submit</button>

    </form>
</article>
