<script lang="ts">
    import { onMount } from "svelte";
    import init, {
        MinWeightsOutput,
        what_weights_unlimited,
    } from "$lib/pkg/wasmww";

    onMount(async () => {
        await init();
    });
    let min_amt: number;
    let min_weights: Array<Uint32Array>;

    let calculate = () => {
        let output = what_weights_unlimited(
            (315 - 45) / 2,
            new Uint32Array([10, 25, 35, 45, 55]),
        );
        if (!output) {
            return;
        }

        min_amt = output.amt;
        min_weights = new Array(output.get_weights_size());
        for (let i = 0; i < output.get_weights_size(); ++i) {
            min_weights[i] = output.get_weights(i);
        }
    };
</script>

<button class="bg-red-300 m-4 p-4 rounded text-lg" on:click={calculate}>
    Click to calculate minimum for 315 lbs !
</button>
{#if min_weights}
    <p>The minimum amount needed is: {min_amt}</p>
    <div>
        {#each min_weights as weights}
            <li>{weights}</li>
        {/each}
    </div>
{/if}
