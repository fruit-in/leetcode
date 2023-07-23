# 96. Unique Binary Search Trees
Given *n*, how many structurally unique **BST's** (binary search trees) that store values 1 ... *n*?

#### Example:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> 5
<strong>Explanation:</strong>
Given <i>n</i> = 3, there are a total of 5 unique BST's:

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
</pre>

## Solutions (Ruby)

### 1. Dynamic Programming
```Ruby
# @param {Integer} n
# @return {Integer}
def num_trees(n)
    dp = [0] * (n + 1)
    dp[0] = 1

    for i in 1..n
        for j in 1..i
            dp[i] += dp[j - 1] * dp[i - j]
        end
    end

    return dp[n]
end
```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }

        dp[n]
    }
}
```
