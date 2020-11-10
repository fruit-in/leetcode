# 946. Validate Stack Sequences
Given two sequences `pushed` and `popped` **with distinct values**, return `true` if and only if this could have been the result of a sequence of push and pop operations on an initially empty stack.

#### Example 1:
<pre>
<b>Input:</b> pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
<b>Output:</b> true
<b>Explanation:</b> We might do the following sequence:
push(1), push(2), push(3), push(4), pop() -> 4,
push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
</pre>

#### Example 2:
<pre>
<b>Input:</b> pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
<b>Output:</b> false
<b>Explanation:</b> 1 cannot be popped before 2.
</pre>

#### Constraints:
* `0 <= pushed.length == popped.length <= 1000`
* `0 <= pushed[i], popped[i] < 1000`
* `pushed` is a permutation of `popped`.
* `pushed` and `popped` have distinct values.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
