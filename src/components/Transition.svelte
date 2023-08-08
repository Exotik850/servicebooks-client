<script lang='ts'>
    import {fade, blur} from 'svelte/transition';

    export let animDur: number = 150;
    export let trans = blur;
</script>

<!-- {#key key} -->
<div in:trans={{duration:animDur, delay:animDur}} out:trans={{duration:animDur}}>
    <slot/>
</div>
<!-- {/key} -->