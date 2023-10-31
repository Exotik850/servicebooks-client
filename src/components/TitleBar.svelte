<script lang="ts">
  // @ts-nocheck

  import { appWindow } from "@tauri-apps/api/window";
  import Dropdown from "./Dropdown.svelte";
  import { fly, blur } from "svelte/transition";
  import { invoke } from "@tauri-apps/api";
  import ErrorPanel from "./ErrorPanel.svelte";

  enum State {
    CLAIM_FORM = "CLAIM_FORM",
    DOCUMENT_UPLOAD = "DOCUMENT_UPLOAD",
    CLAIM_SEARCH = "CLAIM_SEARCH",
  }

  let href = "/";

  export let state;

  let items = [
    { id: 1, label: "Claim Form", action: () => state = State.CLAIM_FORM },
    {
      id: 2,
      label: "Document Upload",
      action: () => state = State.DOCUMENT_UPLOAD,
    },
    {
      id: 3,
      label: "Claim Search",
      action: () => state = State.CLAIM_SEARCH,
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

  async function submitReport() {
    if (!bug_report) {
      return;
    }
    panel.activate();
  }

  function submitPromise() {
    return new Promise((resolve, reject) => {
      let args = { name, email, description }
      invoke("send_email", args)
      .catch((err) => reject(err))
      .then((succ) => {
        name = ""
        email = ""
        description = ""
        resolve(succ);
      })

    })
  }

  let panel;
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
    <dialog open="true" transition:blur>
      <div id="bug-form">
        <article style="width: 95vw">
          <ErrorPanel
            func={submitPromise}
            bind:panel
          />
          <h1>Bug Report</h1>
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

            <div style="display:flex; gap: 5px">
              <button on:click={submitReport}> Submit </button>
              <button on:click={() => (bug_report = false)} class="cancel"> Cancel </button>
            </div>
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
  .cancel {
    background-color: red;
    border:red;
    color:white;
  }
  h5 {
    margin-top: 2px;
  }
</style>
