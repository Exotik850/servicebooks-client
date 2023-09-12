<script lang="ts">
    // @ts-nocheck
    import * as yup from "yup";
    import { quintOut } from "svelte/easing";
    import { tweened } from "svelte/motion";
    import NavButtons from "../components/NavButtons.svelte";
    import Parts from "../components/Parts.svelte";
    import Transition from "../components/Transition.svelte";
    import { invoke } from "@tauri-apps/api";
    import Codes from "./Codes.svelte";

    const progress = tweened(1, {
        duration: 400,
        easing: quintOut,
    });

    $: progress.set((step / 3) * 100);

    const partSchema = yup.object({
        part_number: yup.string().trim().min(1).required(),
        invoice_number: yup.number().required(),
        distributor_number: yup.number().required(),
    });

    const invoiceSchema = yup.object({
        customer_first_name: yup
            .string()
            .trim()
            .min(3)
            .required("Customer First Name is required"),
        customer_last_name: yup
            .string()
            .trim()
            .min(3)
            .required("Customer Last Name is required"),
        customer_address_1: yup
            .string()
            .trim()
            .required("Customer Address is required"),
        customer_city: yup
            .string()
            .trim()
            .required("Customer City is required"),
        customer_zip_code: yup
            .string()
            .trim()
            .required("Customer Zip Code is required"),
        customer_email: yup.string().email(),
        customer_phone_number: yup
            .string()
            .matches(
                "^d{10}$",
                "Phone number must consist of ten digits, no dashes, periods, or other punctuation is allowed"
            )
            .required("Customer Phone Number is required"),
        product_code: yup
            .string()
            .trim()
            .min(8)
            .max(10)
            .required("Appliance Product Code is required"),
        serial_number: yup
            .number()
            .min(10)
            .max(10)
            .required("Appliance Serial Number is required"),
        model_number: yup
            .string()
            .trim()
            .min(8)
            .required("Appliance Model Number is required"),
        purchase_date: yup
            .date()
            .required("Appliance Purchase Date is required"),
        requested_date: yup.date().required("Service Request Date is required"),
        completed_date: yup
            .date()
            .required("Service Completed Date is required"),
        miles_traveled: yup
            .number()
            .min(1)
            .max(999)
            .required("Miles Traveled is required"),
        repair_code: yup
            .number()
            .min(1)
            .max(999)
            .required("Repair Code is required"),
        defect_code: yup
            .number()
            .min(1)
            .max(999)
            .required("Defect Code is required"),
        issue_description: yup
            .string()
            .trim()
            .required("Issue Descripiton is required"),
        service_performed: yup
            .string()
            .trim()
            .required("Service Performed is required"),
        parts: yup.array().of(partSchema),
    });

    let invoice = {
        parts: [],
    };

    let errors = null;
    let getQb = false;
    let getSb = false;

    let notSubmittable;
    $: notSubmittable = (!getQb && !getSb) || (!getQb && !invoice.claim_number);

    $: if (getQb) {
        invoice.claim_number = null;
    }

    let loading = false;

    async function submitClaim() {
        loading = true;
        errors = {};

        try {
            await invoiceSchema.validate(invoice, { abortEarly: false });
            const customer = await invoke("submit_claim", {
                claim: invoice,
                getSb,
            });
            console.log(customer);
        } catch (error) {
            if (error instanceof yup.ValidationError) {
                errors = error.inner.reduce((acc, err) => {
                    return {
                        ...acc,
                        [err.message]: "",
                    };
                }, {});
            } else {
                errors = error;
            }

            return;
        } finally {
            loading = false;
        }

        if (errors) {
            console.error(errors);
        }
    }

    let step = 0;
    let prevStep;
    let numSteps = 3;

    function next() {
        if (step < numSteps) {
            prevStep = step;
            step++;
        }
    }

    function prev() {
        if (step > 0) {
            prevStep = step;
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
                    <label
                        >First Name: <input
                            bind:value={invoice.customer_first_name}
                        /></label
                    >
                    <label
                        >Last Name: <input
                            bind:value={invoice.customer_last_name}
                        /></label
                    >
                </div>
                <label
                    >Street Address: <input
                        bind:value={invoice.customer_address_1}
                    /></label
                >
                <div class="grid">
                    <label
                        >City: <input
                            bind:value={invoice.customer_city}
                        /></label
                    >
                    <label
                        >Zip Code: <input
                            bind:value={invoice.customer_zip_code}
                        /></label
                    >
                    <label
                        >State: <input
                            bind:value={invoice.customer_state}
                        /></label
                    >
                </div>
                <div class="grid">
                    <label
                        >Phone Number: <input
                            bind:value={invoice.customer_phone_number}
                            type="tel"
                        /></label
                    >
                    <label
                        >Email: <input
                            bind:value={invoice.customer_email}
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
                <label
                    >Product Code: <input
                        bind:value={invoice.product_code}
                    /></label
                >
                <label
                    >Model Number: <input
                        bind:value={invoice.model_number}
                    /></label
                >
                <label
                    >Serial Number: <input
                        bind:value={invoice.serial_number}
                        type="number"
                        max="999999999"
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
                <!-- TODO Fix all this ugh gross -->
                <Codes invoice />
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
            <h2>Parts Used</h2>
            <div class="form-section">
                <Parts {invoice} />
            </div>
            {#if loading}
                <article aria-busy="true" />
            {:else if errors != null}
                {#if typeof errors === "object"}
                    {#each Object.entries(errors) as [key, error]}
                        <div color="danger">
                            <input
                                type="text"
                                placeholder={key}
                                readonly
                                aria-invalid="true"
                            />
                        </div>
                    {/each}
                {:else}
                    <div color="danger">
                        <input
                            type="text"
                            placeholder={errors}
                            readonly
                            aria-invalid="true"
                        />
                    </div>
                {/if}
            {/if}
            <div class="form-section">
                <fieldset>
                    <label for="quickbooks">
                        Quickbooks
                        <input
                            type="checkbox"
                            id="quickbooks"
                            bind:checked={getQb}
                        />
                    </label>
                    <label for="servicepower">
                        Servicepower
                        <input
                            type="checkbox"
                            id="servicepower"
                            bind:checked={getSb}
                        />
                    </label>
                </fieldset>
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
    .form-section fieldset {
        display: flex;
        justify-content: space-evenly;
    }
    h2 {
        text-align: center;
        pointer-events: none;
    }
</style>
