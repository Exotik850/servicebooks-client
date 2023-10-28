<script lang="ts">
  // @ts-nocheck

  import { appWindow } from "@tauri-apps/api/window";
  import Dropdown from "./Dropdown.svelte";
  import { fly } from "svelte/transition";

  enum State {
    CLAIM_FORM = "CLAIM_FORM",
    DOCUMENT_UPLOAD = "DOCUMENT_UPLOAD",
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
      label: "Document Upload",
      action: () => changeState(State.DOCUMENT_UPLOAD),
    },
    {
      id: 3,
      label: "Claim Search",
      action: () => changeState(State.CLAIM_SEARCH),
    },
  ];

  const space = 30;

  const handleSelect = (event) => {
    let item = event.detail;
    item.action();
  };

  let email = "";
  let name = "";
  let description = "";
  let bug_report = false;

  function submitReport() {
    console.log("MF TRIED")
  }
</script>

<br />
<br />

<div data-tauri-drag-region class="titlebar">
  <h5 data-tauri-drag-region class="title">ServiceBooks</h5>
  <div class="titlebar-button menu">
    <Dropdown {items} on:select={handleSelect} />
  </div>

  <a
    on:click={() => (bug_report = true)}
    class="titlebar-button"
    id="titlebar-minimize"
    style="right:{space * 3}px"
    {href}
  >
    <i class="material-icons">bug_report</i>
  </a>
{#if bug_report}
  <dialog open="true" transition:fly>
    <div id="bug-form">
      <article style="width: 95vw">
        <div style="display: flex; justify-content: space-evenly;">
          <h1>Bug Report</h1>
          <button
            class="delete"
            on:click|preventDefault={() => (bug_report = false)}
          >
            <i class="material-icons">close</i>
          </button>
        </div>
        <form>
          <label>
            Name:
            <input bind:value={name} />
          </label>

          <label>
            Email:
            <input type="email" bind:value={email} />
          </label>

          <label>
            Description:
            <textarea bind:value={description} style="resize: none;" />
          </label>

          <button on:click={submitReport}> Submit </button>
        </form>
      </article>
    </div>
  </dialog>
{/if}
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
    position: fixed;
    background-color: none;
    /* background-color: var(--primary-focus); */
    top: 0;
    left: 0;
    height: 30px;
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
  .delete {
    background: rgb(126, 21, 21);
    color: rgb(219, 219, 219);
    border: rgb(78, 13, 13);
    /* padding: 0;
    margin-top: auto;
    margin-bottom: auto;
    left: 10px; */
    transition: 0.3s ease;
    justify-content: center;
    align-items: center;
    display: flex;
    width: 45px;
    height: 40px;
    border-radius: 50%;
    box-shadow: none;
  }

  .delete:hover {
    background: darkred;
    transform: scale(1.1);
  }

  .delete:active {
    background: darkred;
    transform: scale(0.9);
  }
  h5 {
    margin-top: 2px;
  }
</style>
