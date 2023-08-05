<script lang=ts>
// @ts-nocheck

    import { appWindow } from "@tauri-apps/api/window";
    import Dropdown from "./Dropdown.svelte";

    enum State {
        CLAIM_FORM = "CLAIM_FORM",
        CLAIM_SEARCH = "CLAIM_SEARCH",
    }
    
    let href = "/";

    export let state;

    function changeState(changeTo) {
      state = changeTo
    }

    let items = [
    { id: 1, label: 'Claim Form', action: () => changeState(State.CLAIM_FORM) },
    { id: 2, label: 'Claim Search', action: () => changeState(State.CLAIM_SEARCH) },
  ];

  const handleSelect = (event) => {
    let item = event.detail;
    item.action();
  };
</script>
  
  <div data-tauri-drag-region class="titlebar">
    <h5 class="title">ServiceBooks</h5>
  
    <!-- <Menu origin="top left">
      <div slot="activator">
          <a 
          {href}
          class="titlebar-button menu">
            <i class="material-icons">menu</i>
          </a>
      </div> 
      <div class="titlebar-button menu">
          
          <Menuitem class="menu" on:click={() => changeState(State.CLAIM_FORM)}>Claim Form</Menuitem>
          <Menuitem class="menu" on:click={() => changeState(State.CLAIM_SEARCH)}>Claim Search</Menuitem>
        </div>
    </Menu> -->

    <div class="titlebar-button menu">
      <Dropdown {items} on:select={handleSelect} />
    </div>
  
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
  <br/>
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
    .title {
      position: fixed;
      left: 40px;
      font-family: "Comfortaa";
      color: antiquewhite;
    }
    .menu {
      font-size: 1.5rem;
      position: absolute;
      top: -7px;
      text-decoration: none;
      text-align: center;
      left: 0px;
      height: 37px;
      width: 35px;
      color: antiquewhite;
    }
    h5 {
      margin-top: 2px;
    }
</style>