<!-- 
    Forms needed:
        Customer:
            Name: First Last
            Address: Address Line, State, City, Zip
            Phone
            Email
        Appliance:
            Product Code
            Model Number
            Serial Number
            Purchase date
        Problem: 
            Defect Code
            Repair Code
            Request Date
        Labor:
            Completion Date
            Miles travelled
            Description of repair

        
 -->
<script lang='ts'>
    // @ts-nocheck
    
    import TitleBar from '../components/TitleBar.svelte'
    import WarrantyForm from '../components/WarrantyForm.svelte';

    
    enum State {
        CLAIM_FORM = "CLAIM_FORM",
        CLAIM_SEARCH = "CLAIM_SEARCH",
    }

    let state = State.CLAIM_FORM;
</script>

<svelte:head>
    <link rel="stylesheet" href="https://fonts.googleapis.com/icon?family=Material+Icons">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css">
</svelte:head>

<TitleBar {state}/>
{#if state === State.CLAIM_FORM}
<WarrantyForm />
{:else if state === State.CLAIM_SEARCH}
<p>peepee</p>
{/if}
<style> 
    
</style>