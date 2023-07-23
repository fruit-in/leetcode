# 1276. Number of Burgers with No Waste of Ingredients
Given two integers `tomatoSlices` and `cheeseSlices`. The ingredients of different burgers are as follows:
* **Jumbo Burger:** 4 tomato slices and 1 cheese slice.
* **Small Burger:** 2 Tomato slices and 1 cheese slice.

Return `[total_jumbo, total_small]` so that the number of remaining `tomatoSlices` equal to 0 and the number of remaining `cheeseSlices` equal to 0. If it is not possible to make the remaining `tomatoSlices` and `cheeseSlices` equal to 0 return `[]`.

#### Example 1:
<pre>
<strong>Input:</strong> tomatoSlices = 16, cheeseSlices = 7
<strong>Output:</strong> [1,6]
<strong>Explanation:</strong> To make one jumbo burger and 6 small burgers we need 4*1 + 2*6 = 16 tomato and 1 + 6 = 7 cheese. There will be no remaining ingredients.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tomatoSlices = 17, cheeseSlices = 4
<strong>Output:</strong> []
<strong>Explanation:</strong> There will be no way to use all ingredients to make small and jumbo burgers.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> tomatoSlices = 4, cheeseSlices = 17
<strong>Output:</strong> []
<strong>Explanation:</strong> Making 1 jumbo burger there will be 16 cheese remaining and making 2 small burgers there will be 15 cheese remaining.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> tomatoSlices = 0, cheeseSlices = 0
<strong>Output:</strong> [0,0]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> tomatoSlices = 2, cheeseSlices = 1
<strong>Output:</strong> [0,1]
</pre>

#### Constraints:
* `0 <= tomatoSlices <= 10^7`
* `0 <= cheeseSlices <= 10^7`

## Solutions (Python)

### 1. Mathematical
```Python
class Solution:
    def numOfBurgers(self, tomatoSlices: int, cheeseSlices: int) -> List[int]:
        if tomatoSlices % 2 == 0 \
                and tomatoSlices >= 2 * cheeseSlices \
                and tomatoSlices <= 4 * cheeseSlices:
            return [
                tomatoSlices // 2 - cheeseSlices,
                2 * cheeseSlices - tomatoSlices // 2
            ]
        return []
```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 == 0
            && tomato_slices >= 2 * cheese_slices
            && tomato_slices <= 4 * cheese_slices
        {
            vec![
                tomato_slices / 2 - cheese_slices,
                2 * cheese_slices - tomato_slices / 2,
            ]
        } else {
            vec![]
        }
    }
}
```
