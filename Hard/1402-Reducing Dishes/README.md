# 1402. Reducing Dishes
A chef has collected data on the `satisfaction` level of his `n` dishes. Chef can cook any dish in 1 unit of time.

**Like-time coefficient** of a dish is defined as the time taken to cook that dish including previous dishes multiplied by its satisfaction level i.e. `time[i] * satisfaction[i]`.

Return *the maximum sum of **like-time coefficient** that the chef can obtain after dishes preparation*.

Dishes can be prepared in **any** order and the chef can discard some dishes to get this maximum value.

#### Example 1:
<pre>
<strong>Input:</strong> satisfaction = [-1,-8,0,5,-9]
<strong>Output:</strong> 14
<strong>Explanation:</strong> After Removing the second and last dish, the maximum total like-time coefficient will be equal to (-1*1 + 0*2 + 5*3 = 14).
Each dish is prepared in one unit of time.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> satisfaction = [4,3,2]
<strong>Output:</strong> 20
<strong>Explanation:</strong> Dishes can be prepared in any order, (2*1 + 3*2 + 4*3 = 20)
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> satisfaction = [-1,-4,-5]
<strong>Output:</strong> 0
<strong>Explanation:</strong> People do not like the dishes. No dish is prepared.
</pre>

#### Constraints:
* `n == satisfaction.length`
* `1 <= n <= 500`
* `-1000 <= satisfaction[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        let mut sum = 0;
        let mut ret = 0;
        satisfaction.sort_unstable();

        for i in (0..satisfaction.len()).rev() {
            sum += satisfaction[i];
            if sum < 0 {
                break;
            }
            ret += sum;
        }

        ret
    }
}
```
