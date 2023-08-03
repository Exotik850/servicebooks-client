<script lang=ts>
// @ts-nocheck

    import { appWindow } from "@tauri-apps/api/window";
    
    let href = "/";
</script>
  
  <div data-tauri-drag-region class="titlebar">
    <h5 class="left">ServiceBooks</h5>
  
    <slot name="left">
        <a 
        {href}
        class="titlebar-button menu">
          <i class="material-icons">menu</i>
        </a>
    </slot>
  
    <a 
      on:click={() => appWindow.minimize()}
      class="titlebar-button" 
      id="titlebar-minimize"
      {href}
    >
      <i class="material-icons">minimize</i>
    </a>
  
    <a
      on:click|preventDefault={() => appWindow.toggleMaximize()}
      class="titlebar-button"
      id="titlebar-maximize"
      {href}
    >
      <i class="material-icons">maximize</i>

    </a>
  
    <a
      on:click={() => appWindow.close()}
      class="titlebar-button"
      id="titlebar-close"
      {href}
    >
      <i class="material-icons">close</i>

    </a>
  
  </div>
  <!-- <br/> -->
  <br/>
<svelte:head/>

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
      text-decoration: none;
      color: antiquewhite;
    }
    .titlebar-button:hover {
      background-color: var(--primary-inverse);
    }
    .left {
      position: fixed;
      left: 30px;
      font-family: "Comfortaa";
      color: antiquewhite;
    }
    .menu {
      font-size: 1.5rem;
      position: absolute;
      top: -3px;
      text-decoration: none;
      left: 0px;
      height: 30px;
      color: antiquewhite;
    }
    h5 {
      margin-top: 2px;
    }
</style>