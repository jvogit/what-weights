mod utils;

use std::{cmp::Reverse, collections::{HashMap, HashSet}};
use wasm_bindgen::prelude::*;

type MinWeights = (usize, HashSet<Vec<usize>>);

#[wasm_bindgen]
pub struct MinWeightsOutput {
    pub amt: usize,
    weights: Vec<Vec<usize>>,
}

#[wasm_bindgen]
impl MinWeightsOutput {
    pub fn get_weights_size(&self) -> usize {
        self.weights.len()
    }

    pub fn get_weights(&self, idx: usize) -> Vec<usize> {
        self.weights[idx].clone()
    }
}

impl From<MinWeights> for MinWeightsOutput {
    fn from(value: MinWeights) -> Self {
        let mut weights: Vec<Vec<usize>> = value.1.into_iter().collect();
        weights.sort_by(|a, b| b.cmp(a));

        Self {
            amt: value.0,
            weights,
        }
    }
}

#[wasm_bindgen]
pub fn what_weights_unlimited(
    target_weight: usize,
    weights: Vec<usize>,
) -> Option<MinWeightsOutput> {
    let mut min_coins: HashMap<usize, MinWeights> =
        HashMap::from([(0, (0, HashSet::from([vec![]])))]);

    for i in 1..=target_weight {
        for weight in weights.iter() {
            if *weight > i || !min_coins.contains_key(&(i - weight)) {
                continue;
            }

            let difference = i - weight;
            let (amt_d, combos_d) = &min_coins[&difference];
            let contains_i = min_coins.contains_key(&i);

            if !contains_i || amt_d + 1 <= min_coins[&i].0 {
                let combos: HashSet<Vec<usize>> = combos_d
                    .clone()
                    .into_iter()
                    .map(|mut v| {
                        v.push(*weight);
                        v.sort_by_key(|w| Reverse(*w));

                        v
                    })
                    .collect();

                if contains_i && amt_d + 1 == min_coins[&i].0 {
                    min_coins.get_mut(&i).unwrap().1.extend(combos);
                } else {
                    min_coins.insert(i, (amt_d + 1, combos));
                }
            }
        }
    }

    return min_coins
        .remove(&target_weight)
        .map(|value| MinWeightsOutput::from(value));
}

#[wasm_bindgen]
pub fn what_weights_limited(
    target_weight: usize,
    weights: Vec<usize>,
    limits: Vec<usize>,
) -> Option<MinWeightsOutput> {
    let mut min_coins: HashMap<usize, MinWeights> =
        HashMap::from([(0, (0, HashSet::from([vec![]])))]);

    for (i, weight) in weights.iter().enumerate() {
        for _ in 0..limits[i] {
            for j in (0..=target_weight).rev() {
                let cand_weight = j + weight;

                if cand_weight > target_weight || !min_coins.contains_key(&j) {
                    continue;
                }

                let (amt_j, combos_j) = &min_coins[&j];
                let contains_cand_weight = min_coins.contains_key(&cand_weight);

                if !contains_cand_weight || amt_j + 1 <= min_coins[&cand_weight].0 {
                    let combos: HashSet<Vec<usize>> = combos_j
                        .clone()
                        .into_iter()
                        .map(|mut v| {
                            v.push(*weight);
                            v.sort_by_key(|w| Reverse(*w));

                            v
                        })
                        .collect();

                    if contains_cand_weight && amt_j + 1 == min_coins[&cand_weight].0 {
                        min_coins.get_mut(&cand_weight).unwrap().1.extend(combos);
                    } else {
                        min_coins.insert(cand_weight, (amt_j + 1, combos));
                    }
                }
            }
        }
    }

    return min_coins
        .remove(&target_weight)
        .map(|value| MinWeightsOutput::from(value));
}
