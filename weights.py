# standard kg
weights_kg = [5, 10, 20, 25, 50, 100, 150, 200, 250]
bar_weight_kg = 200
target_weight_kg = 1870
clip_weight_kg = 0

# standard lbs
weights_lbs = [5, 10, 25,50, 100, 250, 350, 450, 550]
bar_weight_lbs = 450
target_weight_lbs = 4050
clip_weight_lbs = 0
# unlimited amount of each weights
def what_weights(target_weight, weights):
    dp = dict() # [None for _ in range(target_weight + 1)]
    dp[0] = (1, {()})
    for weight in weights:
        dp[weight] = (1, {(weight,)})

    for i in range(weights[0], target_weight + 1):
        for weight in weights:
            difference = i - weight
            if weight > i or difference not in dp:
                continue
            diff_amt, diff_combos = dp[difference]
            amt = diff_amt + 1
            i_amt, _ = dp.setdefault(i, (amt, set()))
            if amt <= i_amt:
                combos = {tuple(sorted(combo + (weight,))) for combo in diff_combos}
                dp[i] = (amt, dp[i][1] | combos if amt == dp[i][0] else combos)

    return dp.get(target_weight, None)

amt, combos = what_weights((target_weight_lbs - bar_weight_lbs - clip_weight_lbs) // 2, weights_lbs)

print(combos)