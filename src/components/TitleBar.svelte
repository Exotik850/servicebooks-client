<script>
    import { appWindow } from '@tauri-apps/api/window'
    let href = "/"
</script>

<!-- <svelte:head> -->
<div data-tauri-drag-region class="titlebar" >
    <div class="titlebar-button" id="titlebar-minimize">
        <a on:click={() => appWindow.minimize()} {href}>
            <img
            src="https://api.iconify.design/mdi:window-minimize.svg"
            alt="minimize"
            />
        </a>
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
        <a on:click|preventDefault={() => appWindow.toggleMaximize()} {href}>
            <img
            src="https://api.iconify.design/mdi:window-maximize.svg"
            alt="maximize"
            />
        </a>
    </div>
    <div class="titlebar-button" id="titlebar-close" >
        <a on:click={() => appWindow.close()} {href}>
            <img
            src="https://api.iconify.design/mdi:close.svg" 
            alt="close"
            />
        </a>
    </div>
</div>
<!-- </svelte:head> -->

<style>
    .titlebar {
      height: 30px;
      user-select: none;
      display: flex;
      justify-content: flex-end;
      position: fixed;
      background-color: var(--primary-inverse);
      top: 0;
      left: 0;
      right: 0;
      padding-bottom: 10px;
      border-radius: 5px 5px 0px 0px;
    }
    .titlebar-button {
      display: inline-flex;
      justify-content: center;
      align-items: center;
      width: 30px;
      height: 30px;
    }
    .titlebar-button:hover {
      background-color: var(--primary-hover);
    }
</style>