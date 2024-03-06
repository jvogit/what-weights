<script lang="ts">
    import type { WeightPlateColor, WeightPlateSize } from "./weighttypes";

    export let weightSize: WeightPlateSize;
    export let weightColor: WeightPlateColor;
</script>

<div class="{weightSize} {weightColor}">
    <slot />
</div>

<style>
    .weight-plate-std {
        width: 25px;
        height: 225px;
        border-style: solid;
        border-width: 1px;
        border-radius: 3px;
    }

    .weight-plate-med {
        width: 15px;
        height: 135px;
        border-style: solid;
        border-width: 1px;
        border-radius: 3px;
    }

    .weight-plate-sml {
        width: 15px;
        height: 100px;
        border-style: solid;
        border-width: 1px;
        border-radius: 3px;
    }

    .weight-plate-red {
        background: rgb(127, 44, 50);
        background: radial-gradient(
            circle,
            rgba(127, 44, 50, 1) 0%,
            rgba(127, 76, 80, 1) 100%
        );
    }

    .weight-plate-blue {
        background: rgba(28, 44, 124);
        background: radial-gradient(
            circle,
            rgba(28, 44, 124, 1) 0%,
            rgba(58, 76, 124, 1) 100%
        );
    }

    .weight-plate-yellow {
        background: rgb(255, 233, 65);
        background: radial-gradient(
            circle,
            rgba(255, 233, 65, 1) 0%,
            rgba(254, 239, 130, 1) 100%
        );
    }

    .weight-plate-green {
        background: rgb(70, 139, 64);
        background: radial-gradient(
            circle,
            rgba(70, 139, 64, 1) 0%,
            rgba(113, 145, 110, 1) 100%
        );
    }

    .weight-plate-white {
        background: rgb(255, 255, 255);
        background: radial-gradient(
            circle,
            rgba(255, 255, 255, 1) 0%,
            rgba(245, 243, 235, 1) 100%
        );
    }

    .weight-plate-black {
        background: rgb(0, 0, 0);
        background: radial-gradient(
            circle,
            rgba(0, 0, 0, 1) 0%,
            rgb(24, 24, 24, 1) 100%
        );
    }
</style>
