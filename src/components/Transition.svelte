<script lang='ts'>
    import {fade, blur} from 'svelte/transition';

    export let animDur: number = 150;
    export let ease = blur;
</script>

<!-- {#key key} -->
<div in:ease={{duration:animDur, delay:animDur}} out:ease={{duration:animDur}}>
    <slot/>
</div>
<!-- {/key} -->