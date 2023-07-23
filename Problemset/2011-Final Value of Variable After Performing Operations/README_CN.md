# 2011. 执行操作后的变量值
存在一种仅支持 4 种操作和 1 个变量 `X` 的编程语言：
* `++X` 和 `X++` 使变量 `X` 的值 **加** `1`
* `--X` 和 `X--` 使变量 `X` 的值 **减** `1`

最初，`X` 的值是 `0`

给你一个字符串数组 `operations` ，这是由操作组成的一个列表，返回执行所有操作后， `X` 的 **最终值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> operations = ["--X","X++","X++"]
<strong>输出:</strong> 1
<strong>解释:</strong> 操作按下述步骤执行：
最初，X = 0
--X：X 减 1 ，X =  0 - 1 = -1
X++：X 加 1 ，X = -1 + 1 =  0
X++：X 加 1 ，X =  0 + 1 =  1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> operations = ["++X","++X","X++"]
<strong>输出:</strong> 3
<strong>解释:</strong> 操作按下述步骤执行：
最初，X = 0
++X：X 加 1 ，X = 0 + 1 = 1
++X：X 加 1 ，X = 1 + 1 = 2
X++：X 加 1 ，X = 2 + 1 = 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> operations = ["X++","++X","--X","X--"]
<strong>输出:</strong> 0
<strong>解释:</strong> 操作按下述步骤执行：
最初，X = 0
X++：X 加 1 ，X = 0 + 1 = 1
++X：X 加 1 ，X = 1 + 1 = 2
--X：X 减 1 ，X = 2 - 1 = 1
X--：X 减 1 ，X = 1 - 1 = 0
</pre>

#### 提示:
* `1 <= operations.length <= 100`
* `operations[i]` 将会是 `"++X"`、`"X++"`、`"--X"` 或 `"X--"`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def finalValueAfterOperations(self, operations: List[str]) -> int:
        return sum(44 - ord(o[1]) for o in operations)
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().map(|o| 44 - o.as_bytes()[1] as i32).sum()
    }
}
```
