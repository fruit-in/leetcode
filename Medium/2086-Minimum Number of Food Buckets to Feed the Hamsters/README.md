# 2086. Minimum Number of Food Buckets to Feed the Hamsters
You are given a **0-indexed** string `hamsters` where `hamsters[i]` is either:

* `'H'` indicating that there is a hamster at index `i`, or
* `'.'` indicating that index `i` is empty.

You will add some number of food buckets at the empty indices in order to feed the hamsters. A hamster can be fed if there is at least one food bucket to its left or to its right. More formally, a hamster at index `i` can be fed if you place a food bucket at index `i - 1` **and/or** at index `i + 1`.

Return *the minimum number of food buckets you should **place at empty indices** to feed all the hamsters or* `-1` *if it is impossible to feed all of them*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/11/01/example1.png)
<pre>
<strong>Input:</strong> hamsters = "H..H"
<strong>Output:</strong> 2
<strong>Explanation:</strong> We place two food buckets at indices 1 and 2.
It can be shown that if we place only one food bucket, one of the hamsters will not be fed.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/11/01/example2.png)
<pre>
<strong>Input:</strong> hamsters = ".H.H."
<strong>Output:</strong> 1
<strong>Explanation:</strong> We place one food bucket at index 2.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/11/01/example3.png)
<pre>
<strong>Input:</strong> hamsters = ".HHH."
<strong>Output:</strong> -1
<strong>Explanation:</strong> If we place a food bucket at every empty index as shown, the hamster at index 2 will not be able to eat.
</pre>

#### Constraints:
* <code>1 <= hamsters.length <= 10<sup>5</sup></code>
* `hamsters[i]` is either`'H'` or `'.'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let hamsters = hamsters
            .replace("H.H", "B")
            .replace(".H", "B")
            .replace("H.", "B");

        if hamsters.contains('H') {
            -1
        } else {
            hamsters.chars().filter(|&c| c == 'B').count() as i32
        }
    }
}
```
