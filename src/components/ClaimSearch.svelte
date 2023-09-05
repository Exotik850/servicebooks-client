<script lang="ts">
    //@ts-nocheck

    import { invoke } from "@tauri-apps/api";
    import Transition from "./Transition.svelte";

    let claimNumber = "";
    let claim = null;
    let error = null;
    let loading = false;
    let getQb = false;
    let getSb = false;


    async function getClaim() {
        error = "";
        loading = true;
        if (claimNumber == "") {
            claim = null;
            return;
        }
        await invoke("get_claim", {
            claimNumber: claimNumber,
            getQb: getQb,
            getSb: getSb,
        })
            .then((respo) => {
                claim = respo;
            })
            .catch((err) => {
                console.error(err);
                error = err;
            })
            .finally(() => {
                loading = false;
            });
    }

    function displayObject(obj, indent = 0) {
        const inc = 10;
        if (obj && typeof obj === "object") {
            return Object.entries(obj)
                .map(([key, value]) => {
                    if (typeof value === "object") {
                        return `<div class="nested" style="padding-left:${
                            indent + inc
                        }px">
                        <p><strong>${key}:</strong></p>
                        ${displayObject(value, indent + inc)}  
                    </div>`;
                    } else {
                        return `<p style="padding-left:${indent}px"><strong>${key}:</strong> ${value}</p>`;
                    }
                })
                .join("");
        } else if (obj !== undefined && obj !== null) {
            return `<p style="padding-left:${indent}px">${obj}</p>`;
        } else {
            return "<br/>";
        }
    }
</script>

<h2>Claim Search</h2>
<article>
    <form>
        <label
            >Claim Number: <input
                type="search"
                bind:value={claimNumber}
            /></label
        >
        Get from:
        <fieldset>
            <label for="quickbooks">
                Quickbooks
                <input
                    type="checkbox"
                    id="quickbooks"
                    on:change={() => (getQb = !getQb)}
                />
            </label>
            <label for="servicepower">
                Servicepower
                <input
                    type="checkbox"
                    id="servicepower"
                    on:change={() => (getSb = !getSb)}
                />
            </label>
        </fieldset>
        <button
            type="submit"
            disabled={(!getQb && !getSb) || !claimNumber}
            on:click|preventDefault={getClaim}>Search</button
        >
    </form>
</article>

{#if error}
    <div class="error">
        <Transition>
            <article>
                <p>{error}</p>
            </article>
        </Transition>
    </div>
{/if}

{#if loading}
<article aria-busy="true" class="secondary"></article>
{:else if claim}
    <div class="claim">
        <Transition>
            <article>
                {@html displayObject(claim)}
            </article>
        </Transition>
    </div>
{/if}

<style>
    h2 {
        text-align: center;
    }
    article {
        width: 95%;
        margin-left: auto;
        margin-right: auto;
    }
    .error article {
        background-color: #af0606;
    }
    .error p {
        color: antiquewhite;
        text-align: center;
        margin: 0 auto;
    }
    fieldset {
        display: flex;
        justify-content: space-evenly;
    }
</style>
