# standard kg
weights_kg = [5, 10, 20, 25, 50, 100, 150, 200]
bar_weight_kg = 200
target_weight_kg = 1400
clip_weight_kg = 0

# standard lbs
weights_lbs = [5, 10, 25, 50, 100, 250, 350, 450, 550]
bar_weight_lbs = 450
target_weight_lbs = 3150
clip_weight_lbs = 0

# unlimited amount of each weights
# coin change with unlimited coins problem
def what_weights(target_weight, weights):
    dp = dict()
    dp[0] = (1, {()})

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

    return dp.get(target_weight, (None, None))

# limited amount of each weight
def what_weights_limits(target_weight, weights, limits):
    dp = dict()
    dp[0] = (0, {()})

    for i, weight in enumerate(weights):
        for _ in range(limits[i]):
            for j in range(target_weight, -1, -1):
                cand_weight = weight + j
                if cand_weight <= target_weight:
                    if j not in dp:
                        continue
                    j_c, j_combos = dp[j]
                    amt = j_c + 1
                    cand_weight_c, _ = dp.get(cand_weight, (float('inf'), set()))
                    
                    if amt <= cand_weight_c:
                        combos = {tuple(sorted(combo + (weight,))) for combo in j_combos}
                        dp[cand_weight] = (amt, dp.get(cand_weight, (amt, set()))[1] | combos if amt == cand_weight_c else combos) 
    
    return dp.get(target_weight, (None, None))

amt, combos = what_weights((target_weight_kg - bar_weight_kg - clip_weight_kg) // 2, weights_kg)
# amt, combos = what_weights_limits((240 - 20) // 2, [10, 20, 25], [2, 2, 2])

print(combos)