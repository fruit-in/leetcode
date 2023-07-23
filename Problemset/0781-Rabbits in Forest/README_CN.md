# 781. 森林中的兔子
森林中，每个兔子都有颜色。其中一些兔子（可能是全部）告诉你还有多少其他的兔子和自己有相同的颜色。我们将这些回答放在 `answers` 数组里。

返回森林中兔子的最少数量。

#### 示例 1:
<pre>
<strong>输入:</strong> answers = [1, 1, 2]
<strong>输出:</strong> 5
<strong>解释:</strong>
两只回答了 "1" 的兔子可能有相同的颜色，设为红色。
之后回答了 "2" 的兔子不会是红色，否则他们的回答会相互矛盾。
设回答了 "2" 的兔子为蓝色。
此外，森林中还应有另外 2 只蓝色兔子的回答没有包含在数组中。
因此森林中兔子的最少数量是 5: 3 只回答的和 2 只没有回答的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> answers = [10, 10, 10]
<strong>输出:</strong> 11
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> answers = []
<strong>输出:</strong> 0
</pre>

#### 说明:
1. `answers` 的长度最大为`1000`。
2. `answers[i]` 是在 `[0, 999]` 范围内的整数。

## 题解 (Ruby)

### 1. 哈希表
```Ruby
# @param {Integer[]} answers
# @return {Integer}
def num_rabbits(answers)
    same_tell = Hash.new(0)
    ret = 0

    for answer in answers
        same_tell[answer + 1] += 1
    end

    same_tell.each do |k, v|
        ret += (v * 1.0 / k).ceil * k
    end

    return ret
end
```

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut same_tell = HashMap::new();

        for answer in answers {
            *same_tell.entry(answer + 1).or_insert(0) += 1;
        }

        same_tell
            .iter()
            .map(|(&k, &v)| (v as f64 / k as f64).ceil() as i32 * k)
            .sum()
    }
}
```
