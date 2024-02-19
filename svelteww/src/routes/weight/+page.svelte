<script lang="ts">
    import BarbellWeight from "$lib/weightplate/BarbellWeight.svelte";
    import {
        WeightPlateColor,
        WeightPlateSize,
    } from "../../lib/weightplate/weighttypes";
    const weightMaps = new Map<number, [WeightPlateSize, WeightPlateColor]>([
        [1, [WeightPlateSize.STD, WeightPlateColor.RED]],
        [2, [WeightPlateSize.STD, WeightPlateColor.BLUE]],
        [3, [WeightPlateSize.STD, WeightPlateColor.YELLOW]],
        [4, [WeightPlateSize.STD, WeightPlateColor.GREEN]],
        [5, [WeightPlateSize.STD, WeightPlateColor.WHITE]],
        [6, [WeightPlateSize.SMALL, WeightPlateColor.RED]],
        [7, [WeightPlateSize.SMALL, WeightPlateColor.BLUE]],
        [8, [WeightPlateSize.SMALL, WeightPlateColor.YELLOW]],
        [9, [WeightPlateSize.SMALL, WeightPlateColor.GREEN]],
        [10, [WeightPlateSize.SMALL, WeightPlateColor.WHITE]],

    ]);
</script>

<BarbellWeight weights={[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]} weightPlateStyleMap={weightMaps} />

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
</style>
