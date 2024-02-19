<script lang="ts">
    import { onMount } from "svelte";
    import init, {
        MinWeightsOutput,
        what_weights_unlimited,
    } from "$lib/pkg/wasmww";
    import { extract_weights } from "./utils";
    import {
        STD_OLYMPIC_KG,
        STD_OLYMPIC_LB,
        type WeightPlateSet,
    } from "./weightplate/weighttypes";
    import BarbellWeight from "./weightplate/BarbellWeight.svelte";

    onMount(async () => {
        await init();
    });

    const weight_plates = [STD_OLYMPIC_KG, STD_OLYMPIC_LB];

    let out: MinWeightsOutput | undefined;
    $: outMinAmt = out?.amt;
    $: outWeights = extract_weights(out);
    let targetWeight: number;
    let selectedWeightPlateSet: WeightPlateSet;
    let selectedBarbellWeight: number;
    let isClipWeightSelected: boolean = false;

    let calculate = () => {
        let inWeight =
            (selectedWeightPlateSet.parseWeight(targetWeight) -
                selectedBarbellWeight) /
            2;
        if (isClipWeightSelected) inWeight -= selectedWeightPlateSet.clipWeight;

        out = what_weights_unlimited(
            inWeight,
            new Uint32Array(selectedWeightPlateSet.weights),
        );
    };
</script>

<h1>Calculate Weights</h1>
<form on:submit|preventDefault={calculate}>
    <h2>Weight Plates Set</h2>
    <select bind:value={selectedWeightPlateSet}>
        {#each weight_plates as weight_plate}
            <option value={weight_plate}>
                {weight_plate.displayName}
            </option>
        {/each}
    </select>
    {#if selectedWeightPlateSet}
        <h2>Target Weight</h2>
        <label>
            Target Weight:
            <input type="number" bind:value={targetWeight} />
            {selectedWeightPlateSet.unit}
        </label>
        <h2>Barbell Weight</h2>
        <select bind:value={selectedBarbellWeight}>
            {#each selectedWeightPlateSet.barbellWeights as barbellWeight}
                <option value={barbellWeight}>
                    {selectedWeightPlateSet.fmtWeightWithUnit(barbellWeight)}
                </option>
            {/each}
        </select>
        <h2>Include Barbell Clip Weight</h2>
        <label>
            <input
                type="checkbox"
                bind:checked={isClipWeightSelected}
                name="clip weight"
            />
            {selectedWeightPlateSet.fmtWeightWithUnit(
                selectedWeightPlateSet.clipWeight,
            )}
        </label>
    {/if}
    <button type="submit" disabled={!selectedWeightPlateSet}>
        Calculate
    </button>
</form>

{#if out}
    <p>The minimum amount needed is: {outMinAmt}</p>
    <div style="width: 17rem;">
        {#each outWeights as weights}
            <BarbellWeight
                weights={weights.sort((a, b) => b - a)}
                weightPlateStyleMap={selectedWeightPlateSet.weightsStyle}
            />
        {/each}
    </div>
{/if}
