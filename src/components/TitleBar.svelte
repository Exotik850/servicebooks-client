<script lang=ts>
// @ts-nocheck

    import { appWindow } from "@tauri-apps/api/window";
    export let leftButtons = [];
    
    let href = "/";
</script>
  
  <div data-tauri-drag-region class="titlebar">
    <h5 class="left">ServiceBooks</h5>
  
    <slot name="left">
      {#each leftButtons as button}
        <a 
          class="titlebar-button"
          on:click={button.onClick}
          href={button.href}
        >
          {@html button.icon}
        </a>
      {/each}
    </slot>
  
    <a 
      on:click={() => appWindow.minimize()}
      class="titlebar-button" 
      id="titlebar-minimize"
      {href}
    >
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </a>
  
    <a
      on:click|preventDefault={() => appWindow.toggleMaximize()}
      class="titlebar-button"
      id="titlebar-maximize"
      {href}
    >
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize" 
      />
    </a>
  
    <a
      on:click={() => appWindow.close()}
      class="titlebar-button"
      id="titlebar-close"
      {href}
    >
      <img 
        src="https://api.iconify.design/mdi:close.svg"
        alt="close" 
      />
    </a>
  
  </div>
  <!-- <br/> -->
  <br/>
<svelte:body/>

<style>
    .titlebar {
      height: 30px;
      user-select: none;
      display: flex;
      justify-content: flex-end;
      position: fixed;
      background-color: var(--primary-focus);
      top: 0;
      left: 0;
      right: 0;
      padding-bottom: 10px;
      user-select: none;
    }
    .titlebar-button {
      display: inline-flex;
      justify-content: center;
      align-items: center;
      width: 30px;
      height: 30px;
    }
    .titlebar-button:hover {
      background-color: var(--primary-inverse);
    }
    .left {
      position: fixed;
      left: 10px;
      font-family: "Comfortaa";
      color: antiquewhite;
    }
</style>