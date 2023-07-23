# 946. 验证栈序列
给定 `pushed` 和 `popped` 两个序列，每个序列中的 **值都不重复**，只有当它们可能是在最初空栈上进行的推入 push 和弹出 pop 操作序列的结果时，返回 `true`；否则，返回 `false` 。

#### 示例 1:
<pre>
<b>输入:</b> pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
<b>输出:</b> true
<b>解释:</b> 我们可以按以下顺序执行：
push(1), push(2), push(3), push(4), pop() -> 4,
push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
</pre>

#### 示例 2:
<pre>
<b>输入:</b> pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
<b>输出:</b> false
<b>解释:</b> 1 不能在 2 之前弹出。
</pre>

#### 提示:
1. `0 <= pushed.length == popped.length <= 1000`
2. `0 <= pushed[i], popped[i] < 1000`
3. `pushed` 是 `popped` 的排列。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} pushed
# @param {Integer[]} popped
# @return {Boolean}
def validate_stack_sequences(pushed, popped)
    stack = []
    i = 0

    for val in pushed
        stack.push(val)
        while i < popped.length and stack.last == popped[i]
            stack.pop
            i += 1
        end
    end

    return stack.empty?
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut i = 0;

        for val in pushed {
            stack.push(val);
            while i < popped.len() && *stack.last().unwrap_or(&-1) == popped[i] {
                stack.pop();
                i += 1;
            }
        }

        stack.is_empty()
    }
}
```
