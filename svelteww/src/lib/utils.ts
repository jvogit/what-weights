import type { MinWeightsOutput } from "./pkg/wasmww";

export const extract_weights = (min_weights: MinWeightsOutput | undefined): Array<Array<number>> => {
    if (!min_weights) {
        return new Array();
    }

    let arr = new Array<Array<number>>(min_weights.get_weights_size());

    for (let i = 0; i < arr.length; ++i) {
        arr[i] = Array.from(min_weights.get_weights(i));
    }

    return arr;
};
