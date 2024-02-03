use std::collections::{HashMap, HashSet};

type MinWeights = (usize, HashSet<Vec<usize>>);

fn what_weights_unlimited(target_weight: usize, weights: &Vec<usize>) -> Option<MinWeights> {
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
                        v.sort();

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

    return min_coins.remove(&target_weight);
}

fn what_weights_limited(
    target_weight: usize,
    weights: Vec<usize>,
    limits: Vec<usize>,
) -> Option<MinWeights> {
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
                            v.sort();

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

    return min_coins.remove(&target_weight);
}

fn main() {
    println!(
        "{:?}",
        what_weights_unlimited((3150 - 450) / 2, &vec![25, 50, 100, 250, 350, 450, 550])
    );

    println!(
        "{:?}",
        what_weights_limited((180 - 20) / 2, vec![10, 15, 20, 25], vec![2, 2, 4, 2])
    );
}
