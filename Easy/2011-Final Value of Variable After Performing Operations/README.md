# 2011. Final Value of Variable After Performing Operations
There is a programming language with only **four** operations and **one** variable `X`:
* `++X` and `X++` **increments** the value of the variable `X` by `1`.
* `--X` and `X--` **decrements** the value of the variable `X` by `1`.

Initially, the value of `X` is `0`.

Given an array of strings `operations` containing a list of operations, return *the **final** value of* `X` *after performing all the operations*.

#### Example 1:
<pre>
<strong>Input:</strong> operations = ["--X","X++","X++"]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The operations are performed as follows:
Initially, X = 0.
--X: X is decremented by 1, X =  0 - 1 = -1.
X++: X is incremented by 1, X = -1 + 1 =  0.
X++: X is incremented by 1, X =  0 + 1 =  1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> operations = ["++X","++X","X++"]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The operations are performed as follows:
Initially, X = 0.
++X: X is incremented by 1, X = 0 + 1 = 1.
++X: X is incremented by 1, X = 1 + 1 = 2.
X++: X is incremented by 1, X = 2 + 1 = 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> operations = ["X++","++X","--X","X--"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The operations are performed as follows:
Initially, X = 0.
X++: X is incremented by 1, X = 0 + 1 = 1.
++X: X is incremented by 1, X = 1 + 1 = 2.
--X: X is decremented by 1, X = 2 - 1 = 1.
X--: X is decremented by 1, X = 1 - 1 = 0.
</pre>

#### Constraints:
* `1 <= operations.length <= 100`
* `operations[i]` will be either `"++X"`, `"X++"`, `"--X"`, or `"X--"`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def finalValueAfterOperations(self, operations: List[str]) -> int:
        return sum(44 - ord(o[1]) for o in operations)
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().map(|o| 44 - o.as_bytes()[1] as i32).sum()
    }
}
```
