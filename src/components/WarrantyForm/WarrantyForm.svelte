<script lang="ts">
  // @ts-nocheck
  import { quintOut } from "svelte/easing";
  import { tweened } from "svelte/motion";
  import NavButtons from "./NavButtons.svelte";
  import Parts from "./Parts.svelte";
  import Transition from "../Transition.svelte";
  import { invoke } from "@tauri-apps/api";
  import Codes from "./Codes.svelte";
  import { ValidationError } from "yup";
  import { invoiceSchema } from "./invoiceSchema";
  import { displayObject } from "../displayObject"

  const progress = tweened(1, {
    duration: 400,
    easing: quintOut,
  });

  $: progress.set((step / 3) * 100);

  let invoice = {
    parts: [],
  };

  let errors = null;
  let getQb = false;
  let getSb = false;
  let loading = false;
  let success = null;

  let notSubmittable;
  $: notSubmittable = (!getQb && !getSb) || (!getQb && !invoice.claim_number);

  $: if (getQb) {
    invoice.claim_number = null;
  }

  async function submitClaim() {
    loading = true;
    success = null
    errors = {};
    console.log("Trying submit");

    try {
      await invoiceSchema.validate(invoice, { abortEarly: false });
      console.log("Validated");
      invoice.phone_number = invoice.phone_number.replaceAll(
        /[^\d]/g,
        ""
      );
      success = await invoke("submit_claim", {
        claim: invoice,
        getSb,
      });
    } catch (error) {
      if (error instanceof ValidationError) {
        errors = error.inner.reduce((acc, err) => {
          return {
            ...acc,
            [err.message]: "",
          };
        }, {});
      } else {
        errors = error;
      }
    } finally {
      loading = false;
    }

    if (errors) {
      console.error(errors);
    } else {
      invoice = {
        parts: [],
      };
    }
  }

  let step = 0;
  let numSteps = 3;

  function next() {
    if (step < numSteps) {
      step++;
    }
  }

  function prev() {
    if (step > 0) {
      step--;
    }
  }
</script>

<!-- 
    Forms needed:
        Customer:
            Name: First Last
            Address: Address Line, State, City, Zip
            Phone
            Email
        Appliance:
            Product Code
            Model Number
            Serial Number
            Purchase date
        Problem: 
            Defect Code
            Repair Code
            Request Date
        Labor:
            Completion Date
            Miles travelled
            Description of repair

        
 -->

<div class="container">
  {#if step === 0}
    <Transition>
      <div class="form-section">
        <h2>Customer Information</h2>
        <div class="grid">
          <label>First Name: <input bind:value={invoice.first_name} /></label>
          <label>Last Name: <input bind:value={invoice.last_name} /></label>
        </div>
        <label>Street Address: <input bind:value={invoice.address_1} /></label>
        <div class="grid">
          <label>City: <input bind:value={invoice.city} /></label>
          <label>State: <input bind:value={invoice.state} /></label>
          <label>Zip Code: <input bind:value={invoice.zip_code} /></label>
        </div>
        <div class="grid">
          <label
            >Phone Number: <input
              bind:value={invoice.phone_number}
              type="tel"
            /></label
          >
          <label
            >Email (Optional): <input
              bind:value={invoice.email}
              type="email"
            /></label
          >
        </div>
      </div>
    </Transition>
  {:else if step === 1}
    <Transition>
      <div class="form-section">
        <h2>Appliance Information</h2>
        <label>Product Code: <input bind:value={invoice.product_code} /></label>
        <label>Model Number: <input bind:value={invoice.model_number} /></label>
        <label
          >Serial Number: <input
            bind:value={invoice.serial_number}
          /></label
        >
        <label
          >Purchase Date: <input
            bind:value={invoice.purchase_date}
            type="date"
          /></label
        >
      </div>
    </Transition>
  {:else if step === 2}
    <Transition>
      <div class="form-section">
        <h2>Labour Information</h2>
        <label
          >Miles Traveled: <input
            bind:value={invoice.miles_traveled}
            type="number"
            min="1"
          /></label
        >
        <div class="grid">
          <label
            >Date Requested: <input
              bind:value={invoice.date_requested}
              type="date"
            /></label
          >
          <label
            >Date Completed: <input
              bind:value={invoice.date_completed}
              type="date"
            /></label
          >
        </div>
        <Codes
          bind:defect_code={invoice.defect_code}
          bind:repair_code={invoice.repair_code}
        />
        <label
          >Description of Issue:<textarea
            rows="4"
            on:resize|preventDefault
            bind:value={invoice.issue_description}
          /></label
        >
        <label
          >Service Performed:<textarea
            rows="4"
            on:resize|preventDefault
            bind:value={invoice.service_performed}
          /></label
        >
        <br />
      </div>
    </Transition>
  {:else if step === 3}
    <Transition>
      <div class="form-section">
        <h2>Parts Used</h2>
        <Parts {invoice} />
      </div>
      <div style="margin: auto; width: 95vw">
        {#if loading}
          <article aria-busy="true" />
        {:else if success != null}
          <article style="background-color: green;">
            <h1>Success!</h1>
            {@html displayObject(success)}
          </article>
        {:else if errors != null}
          <article style="background-color: red;">
            {#if typeof errors === "object"}
              {#each Object.entries(errors) as [key, _]}
                <input type="text" placeholder={key} readonly aria-invalid="true" />
              {/each}
            {:else}
              <input
                type="text"
                placeholder={errors}
                readonly
                aria-invalid="true"
              />
            {/if}
          </article>
        {/if}
      </div>
      <div class="form-section">
        <div class="submit-select">
          <span>Submit To:</span>
          <fieldset>
            <label for="quickbooks">
              Quickbooks
              <input type="checkbox" id="quickbooks" role="switch" bind:checked={getQb} />
            </label>
            <label for="servicepower">
              Servicepower
              <input type="checkbox" id="servicepower" role="switch" bind:checked={getSb} />
            </label>
          </fieldset>
        </div>
        {#if !getQb && getSb}
          <label for="claim_number"
            >Claim Number (Required): <input
              bind:value={invoice.claim_number}
            /></label
          >
        {/if}
        <button
          on:click|preventDefault={submitClaim}
          data-tooltip="Make sure you have everything you need!"
          disabled={notSubmittable}>Submit</button
        >
      </div>
    </Transition>
  {/if}
  <NavButtons {prev} {next} {step} />
  <progress value={$progress} max="100" class="progress" />
</div>

<style>
  .container {
    position: absolute;
    left: 0;
    margin-left: 25px;
  }
  .grid {
    display: flex;
    flex: 1 1 auto;
  }
  .grid label {
    margin: auto;
    width: 50%;
  }
  .progress {
    position: fixed;
    bottom: 10px;
    width: 95%;
    margin: auto;
    z-index: 1;
  }
  button:active {
    transform: scale(0.97);
  }
  .submit-select {
    display: flex;
    flex-direction: column;
    /* align-items: center; */
    justify-content: space-evenly;
    align-items: center;
  }
  .submit-select fieldset {
    display: flex;
  }
  .submit-select label {
    padding-left: 10vw;
    padding-right: 10vw;
  }
  .form-section {
    margin: 10px auto;
    width: 95vw;
  }
  .form-section input textarea {
    pointer-events: all;
  }
  .form-section textarea {
    resize: none;
  }
  h2 {
    text-align: center;
    pointer-events: none;
  }
</style>
