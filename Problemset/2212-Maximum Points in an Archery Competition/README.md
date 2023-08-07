# 2212. Maximum Points in an Archery Competition
Alice and Bob are opponents in an archery competition. The competition has set the following rules:

1. Alice first shoots `numArrows` arrows and then Bob shoots `numArrows` arrows.
2. The points are then calculated as follows:
    1. The target has integer scoring sections ranging from `0` to `11` **inclusive**.
    2. For **each** section of the target with score `k` (in between `0` to `11`), say Alice and Bob have shot <code>a<sub>k</sub></code> and <code>b<sub>k</sub></code> arrows on that section respectively. If <code>a<sub>k</sub> >= b<sub>k</sub></code>, then Alice takes `k` points. If <code>a<sub>k</sub> < b<sub>k</sub></code>, then Bob takes `k` points.
    3. However, if <code>a<sub>k</sub> == b<sub>k</sub> == 0</code>, then **nobody** takes `k` points.

* For example, if Alice and Bob both shot `2` arrows on the section with score `11`, then Alice takes `11` points. On the other hand, if Alice shot `0` arrows on the section with score `11` and Bob shot `2` arrows on that same section, then Bob takes `11` points.

You are given the integer `numArrows` and an integer array `aliceArrows` of size `12`, which represents the number of arrows Alice shot on each scoring section from `0` to `11`. Now, Bob wants to **maximize** the total number of points he can obtain.

Return *the array* `bobArrows` *which represents the number of arrows Bob shot on **each** scoring section from* `0` *to* `11`. The sum of the values in `bobArrows` should equal `numArrows`.

If there are multiple ways for Bob to earn the maximum total points, return **any** one of them.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/24/ex1.jpg)
<pre>
<strong>Input:</strong> numArrows = 9, aliceArrows = [1,1,0,1,0,0,2,1,0,1,2,0]
<strong>Output:</strong> [0,0,0,0,1,1,0,0,1,2,3,1]
<strong>Explanation:</strong> The table above shows how the competition is scored.
Bob earns a total point of 4 + 5 + 8 + 9 + 10 + 11 = 47.
It can be shown that Bob cannot obtain a score higher than 47 points.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/02/24/ex2new.jpg)
<pre>
<strong>Input:</strong> numArrows = 3, aliceArrows = [0,0,1,0,0,0,0,0,0,0,0,2]
<strong>Output:</strong> [0,0,0,0,0,0,0,0,1,1,1,0]
<strong>Explanation:</strong> The table above shows how the competition is scored.
Bob earns a total point of 8 + 9 + 10 = 27.
It can be shown that Bob cannot obtain a score higher than 27 points.
</pre>

#### Constraints:
* <code>1 <= numArrows <= 10<sup>5</sup></code>
* `aliceArrows.length == bobArrows.length == 12`
* `0 <= aliceArrows[i], bobArrows[i] <= numArrows`
* `sum(aliceArrows[i]) == numArrows`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let mut max_points = 0;
        let mut bob_arrows = vec![];

        for x in 1..2_i32.pow(12) {
            let mut points = 0;
            let mut arrows = vec![0; 12];
            let mut arrows_sum = 0;

            for i in 0..12 {
                if x & (1 << i) != 0 {
                    points += i;
                    arrows[i] = alice_arrows[i] + 1;
                    arrows_sum += arrows[i];
                }
            }

            if points > max_points && arrows_sum <= num_arrows {
                max_points = points;
                arrows[0] += num_arrows - arrows_sum;
                bob_arrows = arrows;
            }
        }

        bob_arrows
    }
}
```
