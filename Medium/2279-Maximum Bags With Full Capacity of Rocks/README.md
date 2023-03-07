# 2279. Maximum Bags With Full Capacity of Rocks
You have `n` bags numbered from `0` to `n - 1`. You are given two **0-indexed** integer arrays `capacity` and `rocks`. The <code>i<sup>th</sup></code> bag can hold a maximum of `capacity[i]` rocks and currently contains `rocks[i]` rocks. You are also given an integer `additionalRocks`, the number of additional rocks you can place in **any** of the bags.

Return *the **maximum** number of bags that could have full capacity after placing the additional rocks in some bags*.

#### Example 1:
<pre>
<strong>Input:</strong> capacity = [2,3,4,5], rocks = [1,2,4,4], additionalRocks = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Place 1 rock in bag 0 and 1 rock in bag 1.
The number of rocks in each bag are now [2,3,4,4].
Bags 0, 1, and 2 have full capacity.
There are 3 bags at full capacity, so we return 3.
It can be shown that it is not possible to have more than 3 bags at full capacity.
Note that there may be other ways of placing the rocks that result in an answer of 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> capacity = [10,2,2], rocks = [2,2,0], additionalRocks = 100
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Place 8 rocks in bag 0 and 2 rocks in bag 2.
The number of rocks in each bag are now [10,2,2].
Bags 0, 1, and 2 have full capacity.
There are 3 bags at full capacity, so we return 3.
It can be shown that it is not possible to have more than 3 bags at full capacity.
Note that we did not use all of the additional rocks.
</pre>

#### Constraints:
* `n == capacity.length == rocks.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= capacity[i] <= 10<sup>9</sup></code>
* `0 <= rocks[i] <= capacity[i]`
* <code>1 <= additionalRocks <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remains = (0..rocks.len())
            .map(|i| capacity[i] - rocks[i])
            .collect::<Vec<_>>();
        let mut additional_rocks = additional_rocks;
        let mut ret = 0;
        remains.sort_unstable();

        for remain in remains {
            if remain > additional_rocks {
                break;
            }

            additional_rocks -= remain;
            ret += 1;
        }

        ret
    }
}
```
