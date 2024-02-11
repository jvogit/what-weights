<script lang="ts">
    import { onMount } from "svelte";
    import init, {
        MinWeightsOutput,
        what_weights_unlimited,
    } from "$lib/pkg/wasmww";
    import { extract_weights } from "./utils";

    onMount(async () => {
        await init();
    });

    let out: MinWeightsOutput | undefined;
    $: out_min_amt = out?.amt;
    $: out_weights = extract_weights(out);

    let calculate = () => {
        let in_weight =
            target_weight * 10 - selected_weight_plates.barbell_weight;
        if (selected_clip_weight)
            in_weight -= selected_weight_plates.clip_weight;

        in_weight /= 2;

        out = what_weights_unlimited(
            in_weight,
            new Uint32Array(selected_weight_plates.weights),
        );
    };

    let target_weight: number;
    let selected_weight_plates: any | undefined;

    const STD_KG = {
        id: 1,
        display_name: "Standard KGs Plates",
        unit: "kg",
        barbell_weight: 200,
        clip_weight: 50,
        weights: [5, 10, 15, 20, 25, 50, 100, 150, 200, 250],
    };
    const STD_LBS = {
        id: 2,
        display_name: "LB plates",
        unit: "lb",
        barbell_weight: 450,
        clip_weight: 100,
        weights: [25, 50, 100, 250, 350, 450, 550],
    };

    let weight_plates = [STD_KG, STD_LBS];
    let selected_clip_weight: boolean = false;
</script>

<h1>Calculate Weights</h1>
<form on:submit|preventDefault={calculate}>
    <label>
        Target Weight:
        <input type="number" bind:value={target_weight} />
    </label>
    <h2>Weight Plates Set</h2>
    <select bind:value={selected_weight_plates}>
        {#each weight_plates as weight_plate}
            <option value={weight_plate}>
                {weight_plate.display_name}
            </option>
        {/each}
    </select>
    {#if selected_weight_plates}
        <h2>Barbell Weight</h2>
        {selected_weight_plates.barbell_weight / 10} {selected_weight_plates.unit}
        <h2>Include Barbell Clip Weight</h2>
        <label>
            <input
                type="checkbox"
                bind:checked={selected_clip_weight}
                name="clip weight"
            />
            {selected_weight_plates.clip_weight / 20} {selected_weight_plates.unit}
        </label>
    {/if}
    <button type="submit" disabled={!selected_weight_plates}>
        Calculate
    </button>
</form>

{#if out}
    <p>The minimum amount needed is: {out_min_amt}</p>
    <div>
        {#each out_weights as weights}
            <li>{weights}</li>
        {/each}
    </div>
{/if}
