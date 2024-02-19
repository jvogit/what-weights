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

    const getWeightPlateSize = (weight: number) => {
        return weightPlateStyleMap.get(weight)![0];
    };

    const getWeightPlateColor = (weight: number) => {
        return weightPlateStyleMap.get(weight)![1];
    };
</script>

<div class="container">
    <div class="barbell-collar" />
    {#each weights as weight}
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

    .barbell-clip {
        
    }
</style>
