<script lang="ts">

    let claimNumber = '';
</script>

<h2>Claim Search</h2>
<article>
    <label>Claim Number: <input bind:value={claimNumber}/></label>
</article>

<style>
    h2 {
        text-align: center;
    }
    article {
        width: 95%;
        margin-left: auto;
        margin-right: auto;
    }
</style>