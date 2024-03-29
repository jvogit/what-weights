<script lang="ts">
    import WeightPlate from "$lib/weightplate/WeightPlate.svelte";
    import {
        WeightPlateColor,
        WeightPlateSize,
    } from "$lib/weightplate/weighttypes";

    export let weights: Array<number>;
    export let weightPlateStyleMap: Map<
        number,
        [WeightPlateSize, WeightPlateColor]
    >;
    export let clipped: boolean;

    const getWeightPlateSize = (weight: number) => {
        return weightPlateStyleMap.get(weight)![0];
    };

    const getWeightPlateColor = (weight: number) => {
        return weightPlateStyleMap.get(weight)![1];
    };

    $: clipped_weights = weights.filter(
        (weight) => getWeightPlateSize(weight) != WeightPlateSize.SMALL,
    );
    $: unclipped_weights = weights.filter(
        (weight) => getWeightPlateSize(weight) === WeightPlateSize.SMALL,
    );
</script>

<div class="container">
    <div class="barbell-shaft" />
    <div class="barbell-collar" />
    {#each clipped_weights as weight}
        <WeightPlate
            weightSize={getWeightPlateSize(weight)}
            weightColor={getWeightPlateColor(weight)}
        />
    {/each}
    {#if clipped}
        <div class="barbell-clip" />
    {/if}
    {#each unclipped_weights as weight}
        <WeightPlate
            weightSize={getWeightPlateSize(weight)}
            weightColor={getWeightPlateColor(weight)}
        />
    {/each}
    <div class="barbell-sleeve" />
</div>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .barbell-clip {
        width: 35px;
        height: 55px;
        border-radius: 2px;
        background-image: linear-gradient(
            to right,
            silver 0 50%,
            gray 50% 100%
        );
    }

    .barbell-sleeve {
        flex-grow: 1;
        height: 25px;
        background-image: linear-gradient(to right, silver, gray);
    }

    .barbell-collar {
        width: 20px;
        height: 50px;
        background-image: linear-gradient(to right, silver, gray);
    }

    .barbell-shaft {
        width: 30px;
        height: 15px;
        background-image: linear-gradient(to right, silver, gray);
    }
</style>
