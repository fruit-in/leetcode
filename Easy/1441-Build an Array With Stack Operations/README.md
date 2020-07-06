# 1441. Build an Array With Stack Operations
Given an array `target` and an integer `n`. In each iteration, you will read a number from  `list = {1,2,3..., n}`.

Build the `target` array using the following operations:
* **Push**: Read a new element from the beginning `list`, and push it in the array.
* **Pop**: delete the last element of the array.
* If the target array is already built, stop reading more elements.

You are guaranteed that the target array is strictly increasing, only containing numbers between 1 to `n` inclusive.

Return the operations to build the target array.

You are guaranteed that the answer is unique.

#### Example 1:
<pre>
<strong>Input:</strong> target = [1,3], n = 3
<strong>Output:</strong> ["Push","Push","Pop","Push"]
<strong>Explanation:</strong>
Read number 1 and automatically push in the array -> [1]
Read number 2 and automatically push in the array then Pop it -> [1]
Read number 3 and automatically push in the array -> [1,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = [1,2,3], n = 3
<strong>Output:</strong> ["Push","Push","Push"]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = [1,2], n = 4
<strong>Output:</strong> ["Push","Push"]
<strong>Explanation:</strong> You only need to read the first 2 numbers and stop.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> target = [2,3,4], n = 4
<strong>Output:</strong> ["Push","Pop","Push","Push","Push"]
</pre>

#### Constraints:
* `1 <= target.length <= 100`
* `1 <= target[i] <= 100`
* `1 <= n <= 100`
* `target` is strictly increasing.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut i = 1;
        let mut ret = vec![];

        for num in target {
            for _ in 0..(num - i) {
                ret.push("Push".to_string());
                ret.push("Pop".to_string());
            }
            ret.push("Push".to_string());
            i = num + 1;
        }

        ret
    }
}
```
