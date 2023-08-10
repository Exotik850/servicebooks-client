<script lang="ts">
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
    state = changeTo;
  }

  let items = [
    { id: 1, label: "Claim Form", action: () => changeState(State.CLAIM_FORM) },
    {
      id: 2,
      label: "Claim Search",
      action: () => changeState(State.CLAIM_SEARCH),
    },
  ];

  const space = 30;

  const handleSelect = (event) => {
    let item = event.detail;
    item.action();
  };
</script>

<div data-tauri-drag-region class="titlebar">
  <h5 class="title">ServiceBooks</h5>
  <div class="titlebar-button menu">
    <Dropdown {items} on:select={handleSelect} />
  </div>

  <a
    on:click={() => appWindow.minimize()}
    class="titlebar-button"
    id="titlebar-minimize"
    style="right:{space * 2}px"
    {href}
  >
    <i class="material-icons">minimize</i>
  </a>

  <a
    on:click|preventDefault={() => appWindow.toggleMaximize()}
    class="titlebar-button"
    id="titlebar-maximize"
    style="right:{space}px"
    {href}
  >
    <i class="material-icons">maximize</i>
  </a>

  <a
    on:click={() => appWindow.close()}
    class="titlebar-button"
    id="titlebar-close"
    style="right:0"
    {href}
  >
    <i class="material-icons">close</i>
  </a>

  <br />
  <br />
</div>

<style>
  .titlebar {
    height: 99%;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: sticky;
    background-color: none;
    /* background-color: var(--primary-focus); */
    top: 0;
    left: 0;
    right: 0;
    user-select: none;
    border-radius: 10px;
    z-index: 2;
  }
  .titlebar-button {
    position: absolute;
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
