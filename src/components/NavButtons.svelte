<script>
// @ts-nocheck

    export let prev;
    export let next;
    export let step;
</script>

<div class="form-navigation">
    <button class="nav" on:click|preventDefault={prev} disabled={step === 0}>&larr;</button>
    <button class="nav" on:click|preventDefault={next} disabled={step === 3}>&rarr;</button>
</div>

<style>
    .form-navigation {
        display: flex;
        position:fixed;
        gap: 20px;
        bottom: 10px;
        margin: 0 auto;
        width: 95vw;
        justify-content: space-evenly;
        pointer-events: none;
    }
    .nav {
        background: none;
        border: none;
        font-size: 2rem;
        color:white;
        opacity: 0.5;
        transition: 0.3s ease;
        width: 10%;
        pointer-events: all;
        text-align: center;
        border-radius: 50%;
    }
    .nav:hover {
        opacity: 1;
        transform: scale(1.1);
    }
    .nav:active {
        transform: scale(0.97);
    }
</style>