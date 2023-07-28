<script lang='ts'>
    import {slide} from 'svelte/transition';

    export let key: String;
    export let duration: number = 300;
</script>

{#key key}
<div transition:slide={{duration}}>
    <slot/>
</div>
{/key}