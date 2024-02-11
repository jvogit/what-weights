import type { MinWeightsOutput } from "./pkg/wasmww";

export const extract_weights = (min_weights: MinWeightsOutput | undefined) => {
    if (!min_weights) {
        return new Array();
    }

    let arr = new Array<Uint32Array>(min_weights.get_weights_size());

    for (let i = 0; i < arr.length; ++i) {
        arr[i] = min_weights.get_weights(i);
    }

    return arr;
};
