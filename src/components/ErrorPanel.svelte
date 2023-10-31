<script lang="ts">
  interface response {
    error?: string | object;
    success?: string;
  }
  export let func: Function;
  let msg: response = {
    error: undefined,
    success: undefined,
  };

  let loading = false;

  export const panel = {
    setError(err: string | object) {
      msg.error = err
    },
    setSuccess(success: string) {
      msg.success = success
    },
    activate() {
      loading = true;
      msg = {
        error: undefined,
        success: undefined,
      };

      func()
        .then((succ: string) => (msg.success = succ))
        .catch((err: string | object) => (msg.error = err))
        .finally(() => loading = false);
    },
  };
</script>

{#if loading}
  <article aria-busy="true" />
{:else if msg.error !== undefined}
  {#if typeof msg.error === "object"}
    {#each Object.entries(msg.error) as [key, _]}
      <input type="text" placeholder={key} readonly aria-invalid="true" />
    {/each}
  {:else}
    <input type="text" placeholder={msg.error} readonly aria-invalid="true" />
  {/if}
{:else if msg.success !== undefined}
  <article style="background-color: green;">
    <h1>Success!</h1>
    {msg.success}
  </article>
{/if}

<style>
</style>
