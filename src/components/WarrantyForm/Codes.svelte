<script lang=ts>
     export let defect_code: number;
     export let repair_code: number;
</script>

<div class="grid">
    <label
        >Defect Code: <select bind:value={defect_code}>
            <option value="0" selected />
            <option value="13">Not heating</option>
            <option value="14">Over heating</option>
            <option value="15">Noise</option>
            <option value="16">Damaged Laundry</option>
            <option value="17">Timer</option>
            <option value="18">No Power</option>
            <option value="21">Leaking</option>
            <option value="22"
                >Water Issue (Temp, Pressure, Etc)
            </option>
            <option value="23">Door Lock</option>
            <option value="19">Error Code</option>
            <option value="20">Other (Comments Needed)</option>
        </select>
    </label>
    <label
        >Repair/Job Code: <select
            bind:value={repair_code}
        >
            <option value="0" selected />
            <option value="1">Blower Wheel / Motor</option>
            <option value="2">Customer Instruct</option>
            <option value="10">Defective Cylinder</option>
            <option value="11">Defective Door Boot</option>
            <option value="12">Defective Door Lock</option>
            <option value="13">Defective FEC</option>
            <option value="14"
                >Defective Front End Control</option
            >
            <option value="15">Defective Glide</option>
            <option value="16">Defective Harness</option>
            <option value="17">Defective Hose</option>
            <option value="18">Defective Invertor Board</option>
            <option value="19"
                >Defective Lid Lock / Catch</option
            >
            <option value="20">Defective Motor Switch</option>
            <option value="21">Defective Output Board</option>
            <option value="22"
                >Defective Output Board / FEC</option
            >
            <option value="23"
                >Defective Output Board / FEC / Timer</option
            >
            <option value="24"
                >Defective Power Chord / Harness</option
            >
            <option value="25">Defective Pressure Sensor</option
            >
            <option value="26"
                >Defective Pressure Sensor / Harness</option
            >
            <option value="27">Defective Pump</option>
            <option value="28">Defective Roller / Shaft</option>
            <option value="29"
                >Defective Thermostat / Thermistor</option
            >
            <option value="3">Defective Balance Ring</option>
            <option value="30">Defective Washtub</option>
            <option value="31">Defective Water Valve</option>
            <option value="32"
                >Defective / Broken Agitator</option
            >
            <option value="33"
                >Defective / Broken Clothes Gaurd</option
            >
            <option value="34"
                >Defective / Stretched / Worn Belt</option
            >
            <option value="35">GFCI / Breaker</option>
            <option value="36">High Heat Concern</option>
            <option value="37">Install Related</option>
            <option value="38"
                >Lid Alignment / Adjustment</option
            >
            <option value="39">Loose Bolt / Screw</option>
            <option value="4">Defective Seal</option>
            <option value="40">No Advance / Faulty Motor</option
            >
            <option value="41">Noisy Bearing</option>
            <option value="42">Noisy Motor</option>
            <option value="43">Noisy Operation</option>
            <option value="44">Noisy Door Boot</option>
            <option value="45">Open Contact</option>
            <option value="46">Open Element</option>
            <option value="47"
                >Open Ignitor / Gas Valve Coil</option
            >
            <option value="48"
                >Open Thermal Fuse / Thermostat / Thermistor</option
            >
            <option value="5">Defective Air Dome/Tubing</option>
            <option value="50"
                >Poor Door Alignment / Hinge</option
            >
            <option value="51">Roller / Shaft Issue</option>
            <option value="52">Shorted Element</option>
            <option value="53">Stuck Relay</option>
            <option value="54"
                >Timer / Control Board / Switch Defective</option
            >
            <option value="6">Defective Bearing</option>
            <option value="7">Defective Belt</option>
            <option value="8">Defective Belt Idler</option>
            <option value="9">Defective Cord / Harness</option>
            <option value="49">Other (Comments Needed)</option>
        </select>
    </label>
</div>

<style>
    .grid {
        display: flex;
        flex: 1 1 auto;
    }
    .grid label {
        margin: auto;
        width: 50%;
    }
</style>