# 2105. Watering Plants II
Alice and Bob want to water `n` plants in their garden. The plants are arranged in a row and are labeled from `0` to `n - 1` from left to right where the <code>i<sup>th</sup></code> plant is located at `x = i`.

Each plant needs a specific amount of water. Alice and Bob have a watering can each, **initially full**. They water the plants in the following way:

* Alice waters the plants in order from **left to right**, starting from the <code>0<sup>th</sup></code> plant. Bob waters the plants in order from **right to left**, starting from the <code>(n - 1)<sup>th</sup></code> plant. They begin watering the plants **simultaneously**.
* It takes the same amount of time to water each plant regardless of how much water it needs.
* Alice/Bob **must** water the plant if they have enough in their can to **fully** water it. Otherwise, they **first** refill their can (instantaneously) then water the plant.
* In case both Alice and Bob reach the same plant, the one with **more** water currently in his/her watering can should water this plant. If they have the same amount of water, then Alice should water this plant.

Given a **0-indexed** integer array `plants` of `n` integers, where `plants[i]` is the amount of water the <code>i<sup>th</sup></code> plant needs, and two integers `capacityA` and `capacityB` representing the capacities of Alice's and Bob's watering cans respectively, return *the **number of times** they have to refill to water all the plants*.

#### Example 1:
<pre>
<strong>Input:</strong> plants = [2,2,3,3], capacityA = 5, capacityB = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong>
- Initially, Alice and Bob have 5 units of water each in their watering cans.
- Alice waters plant 0, Bob waters plant 3.
- Alice and Bob now have 3 units and 2 units of water respectively.
- Alice has enough water for plant 1, so she waters it. Bob does not have enough water for plant 2, so he refills his can then waters it.
So, the total number of times they have to refill to water all the plants is 0 + 0 + 1 + 0 = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> plants = [2,2,3,3], capacityA = 3, capacityB = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- Initially, Alice and Bob have 3 units and 4 units of water in their watering cans respectively.
- Alice waters plant 0, Bob waters plant 3.
- Alice and Bob now have 1 unit of water each, and need to water plants 1 and 2 respectively.
- Since neither of them have enough water for their current plants, they refill their cans and then water the plants.
So, the total number of times they have to refill to water all the plants is 0 + 1 + 1 + 0 = 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> plants = [5], capacityA = 10, capacityB = 8
<strong>Output:</strong> 0
<strong>Explanation:</strong>
- There is only one plant.
- Alice's watering can has 10 units of water, whereas Bob's can has 8 units. Since Alice has more water in her can, she waters this plant.
So, the total number of times they have to refill is 0.
</pre>

#### Constraints:
* `n == plants.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= plants[i] <= 10<sup>6</sup></code>
* <code>max(plants[i]) <= capacityA, capacityB <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
        let n = plants.len();
        let mut water_a = capacity_a;
        let mut water_b = capacity_b;
        let mut ret = 0;

        for i in 0..n / 2 {
            if water_a < plants[i] {
                water_a = capacity_a;
                ret += 1;
            }
            if water_b < plants[n - 1 - i] {
                water_b = capacity_b;
                ret += 1;
            }

            water_a -= plants[i];
            water_b -= plants[n - 1 - i];
        }

        ret + (n % 2 == 1 && water_a.max(water_b) < plants[n / 2]) as i32
    }
}
```
