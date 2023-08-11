<script lang="ts">
    // @ts-nocheck
    import * as yup from "yup";
    import { quintOut } from "svelte/easing";
    import { tweened } from "svelte/motion";
    import NavButtons from "../components/NavButtons.svelte";
    import Parts from "../components/Parts.svelte";
    import Transition from "../components/Transition.svelte";
    import { invoke } from "@tauri-apps/api";

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
        customer_first_name: yup.string().trim().min(3).required(),
        customer_last_name: yup.string().trim().min(3).required(),
        customer_address_1: yup.string().trim().required(),
        customer_state: yup.string().trim().required(),
        customer_city: yup.string().trim().required(),
        customer_zip_code: yup.string().trim().required(),
        customer_email: yup.string().email(),
        customer_phone_number: yup
            .string()
            .matches("^(+0?1s)?(?d{3})?[s.-]d{3}[s.-]d{4}$"),
        product_code: yup.string().trim().min(8).max(10).required(),
        serial_number: yup.number().min(10).max(10).required(),
        model_number: yup.string().trim().min(8).required(),
        purchase_date: yup.date().required(),
        requested_date: yup.date().required(),
        completed_date: yup.date().required(),
        miles_traveled: yup.number().min(1).max(999).required(),
        repair_code: yup.number().min(1).max(999),
        defect_code: yup.number().min(1).max(999),
        issue_description: yup.string().trim().required(),
        service_performed: yup.string().trim().required(),
        parts: yup.array().of(partSchema),
    });

    let invoice = {
        parts: [],
    };

    let errors = {};

    async function submitClaim() {
        try {
            // await invoiceSchema.validate(invoice, {abortEarly: false});
            errors = {};
            await invoke("submit_claim", { claim: invoice }).then(
                (customer) => {
                    console.log(customer);
                }
            );
        } catch (error) {
            errors = error.inner.reduce((acc, err) => {
                return { ...acc, [err.path]: err.message };
            }, {});
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
                        >State: <input
                            bind:value={invoice.customer_state}
                        /></label
                    >
                    <label
                        >Zip Code: <input
                            bind:value={invoice.customer_zip_code}
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
                        bind:value={invoice.serial_numbeer}
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
                <div class="grid">
                    <label
                        >Defect Code: <select bind:value={invoice.defect_code}>
                            <option value="20" selected />
                            <option value="13">Not heating</option>
                            <option value="14">Over heating</option>
                            <option value="15">Noise</option>
                            <option value="16">Damaged Laundry</option>
                            <option value="17">Timer</option>
                            <option value="18">No Power</option>
                            <option value="21">Leaking</option>
                            <option value="22"
                                >Water Issue (Temp, Pressure, Etc)</option
                            >
                            <option value="23">Door Lock</option>
                            <option value="19">Error Code</option>
                            <option value="20">Other (Comments Needed)</option>
                        </select>
                    </label>
                    <label
                        >Repair/Job Code: <select
                            bind:value={invoice.repair_code}
                        >
                            <option value="49" selected />
                            <option value="1">Blower Wheel / Motor</option>
                            <option value="2">Customer Instruct</option>
                            <option value="10">Defective Cylinder</option>
                            <option value="11">Defective Door Boot</option>
                            <option value="12">Defective Door Lock</option>
                            <option value="13">Defective FEC</option>
                            <option value="14">Defective Front End Control</option>
                            <option value="15">Defective Glide</option>
                            <option value="16">Defective Harness</option>
                            <option value="17">Defective Hose</option>
                            <option value="18">Defective Invertor Board</option>
                            <option value="19">Defective Lid Lock / Catch</option>
                            <option value="20">Defective Motor Switch</option>
                            <option value="21">Defective Output Board</option>
                            <option value="22">Defective Output Board / FEC</option>
                            <option value="23">Defective Output Board / FEC / Timer</option>
                            <option value="24">Defective Power Chord / Harness</option>
                            <option value="25">Defective Pressure Sensor</option>
                            <option value="26">Defective Pressure Sensor / Harness</option>
                            <option value="27">Defective Pump</option>
                            <option value="28">Defective Roller / Shaft</option>
                            <option value="29">Defective Thermostat / Thermistor</option>
                            <option value="3">Defective Balance Ring</option>
                            <option value="30">Defective Washtub</option>
                            <option value="31">Defective Water Valve</option>
                            <option value="32">Defective / Broken Agitator</option>
                            <option value="33">Defective / Broken Clothes Gaurd</option>
                            <option value="34">Defective / Stretched / Worn Belt</option>
                            <option value="35">GFCI / Breaker</option>
                            <option value="36">High Heat Concern</option>
                            <option value="37">Install Related</option>
                            <option value="38">Lid Alignment / Adjustment</option>
                            <option value="39">Loose Bolt / Screw</option>
                            <option value="4">Defective Seal</option>
                            <option value="40">No Advance / Faulty Motor</option>
                            <option value="41">Noisy Bearing</option>
                            <option value="42">Noisy Motor</option>
                            <option value="43">Noisy Operation</option>
                            <option value="44">Noisy Door Boot</option>
                            <option value="45">Open Contact</option>
                            <option value="46">Open Element</option>
                            <option value="47">Open Ignitor / Gas Valve Coil</option>
                            <option value="48">Open Thermal Fuse / Thermostat / Thermistor</option>
                            <option value="5">Defective Air Dome/Tubing</option>
                            <option value="50">Poor Door Alignment / Hinge</option>
                            <option value="51">Roller / Shaft Issue</option>
                            <option value="52">Shorted Element</option>
                            <option value="53">Stuck Relay</option>
                            <option value="54">Timer / Control Board / Switch Defective</option>
                            <option value="6">Defective Bearing</option>
                            <option value="7">Defective Belt</option>
                            <option value="8">Defective Belt Idler</option>
                            <option value="9">Defective Cord / Harness</option>
                            <option value="49">Other (Comments Needed)</option>
                        </select>
                    </label>
                </div>
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
                <br/>
            </div>
        </Transition>
    {:else if step === 3}
        <Transition>
            <div class="form-section">
                <h2>Parts Used</h2>
                <Parts {invoice} />
            </div>
            {#if JSON.stringify(errors) != "{}"}
                {#each Object.entries(errors) as [key, error]}
                    <div color="danger">
                        <h4>{key}</h4>
                        <p>{error}</p>
                    </div>
                {/each}
            {/if}
            <div class="form-section">
                <button
                    on:click|preventDefault={submitClaim}
                    data-tooltip="Make sure you have everything you need!"
                    >Submit</button
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
    h2 {
        text-align: center;
        pointer-events: none;
    }
</style>
