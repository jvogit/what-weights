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

<div style="width: 17rem;">
    <BarbellWeight
        weights={[1, 2, 3, 4]}
        weightPlateStyleMap={weightMaps}
    />
</div>
