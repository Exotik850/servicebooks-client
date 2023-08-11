<script>
  //@ts-nocheck
  import { createEventDispatcher } from "svelte";
  import { onMount } from "svelte";
  import { fly } from "svelte/transition";

  let isOpen = false;
  let dropdownRef;

  // This will dispatch an event to the parent component
  const dispatch = createEventDispatcher();

  onMount(() => {
    const handleClickOutside = (event) => {
      if (dropdownRef && !dropdownRef.contains(event.target)) {
        isOpen = false;
      }
    };

    document.addEventListener("click", handleClickOutside);
    return () => {
      document.removeEventListener("click", handleClickOutside);
    };
  });

  export let items = [];

  const handleItemClick = (item) => {
    dispatch("select", item);
    isOpen = false;
  };
</script>

<div class="dropdown" bind:this={dropdownRef}>
  <!-- svelte-ignore a11y-missing-attribute -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <a on:click={() => (isOpen = !isOpen)} class="dropdown-button">
    <i class="material-icons">menu</i>
  </a>

  {#if isOpen}
    <div class="dropdown-menu" transition:fly={{ x: -200 }}>
      {#each items as item (item.id)}
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="dropdown-item" on:click={() => handleItemClick(item)}>
          {item.label}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown-button {
    color: antiquewhite;
  }

  .dropdown {
    z-index: 1;
    width: 30px;
    height: 30px;
  }

  .dropdown-menu {
    position: fixed;
    left: 5px;
    top: 5px;
    background-color: var(--primary);
    color: var(--background-color);
    box-shadow: 0px 8px 16px 0px rgba(0, 0, 0, 0.2);
    z-index: 99;
    border-radius: 5px;
    overflow: hidden;
    cursor: pointer;
    pointer-events: all;
  }

  .dropdown-item {
    padding: 12px 16px;
    cursor: pointer;
    width: 200px;
  }

  .dropdown-item:hover {
    background-color: var(--primary-inverse);
  }
</style>
