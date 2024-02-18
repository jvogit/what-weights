<script lang="ts">
    import WeightPlate from "$lib/WeightPlate.svelte";
</script>

<div class="container">
    <div class="barbell-collar" />
    <WeightPlate weight={1} />
    <WeightPlate weight={2} />
    <div class="barbell-sleeve" />
</div>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
        gap: 0.2rem;
    }

    .barbell-sleeve {
        flex-grow: 1;
        height: 1.2rem;
        background-color: #000000;
    }

    .barbell-collar {
        width: 1rem;
        height: 2.5rem;
        background-color: #000000;
    }
</style>
