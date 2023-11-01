# 2007. 从双倍数组中还原原数组
一个整数数组 `original` 可以转变成一个 **双倍** 数组 `changed` ，转变方式为将 `original` 中每个元素 **值乘以 2** 加入数组中，然后将所有元素 **随机打乱** 。

给你一个数组 `changed` ，如果 `change` 是 **双倍** 数组，那么请你返回 `original`数组，否则请返回空数组。`original` 的元素可以以 **任意** 顺序返回。

#### 示例 1:
<pre>
<strong>输入:</strong> changed = [1,3,4,2,6,8]
<strong>输出:</strong> [1,3,4]
<strong>解释:</strong> 一个可能的 original 数组为 [1,3,4] :
- 将 1 乘以 2 ，得到 1 * 2 = 2 。
- 将 3 乘以 2 ，得到 3 * 2 = 6 。
- 将 4 乘以 2 ，得到 4 * 2 = 8 。
其他可能的原数组方案为 [4,3,1] 或者 [3,1,4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> changed = [6,3,0,1]
<strong>输出:</strong> []
<strong>解释:</strong> changed 不是一个双倍数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> changed = [1]
<strong>输出:</strong> []
<strong>解释:</strong> changed 不是一个双倍数组。
</pre>

#### 提示:
* <code>1 <= changed.length <= 10<sup>5</sup></code>
* <code>0 <= changed[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;
        let mut count = HashMap::new();
        let mut original = vec![];

        changed.sort_unstable();

        for num in &changed {
            if num % 2 == 0 && *count.get(&(num / 2)).unwrap_or(&0) > 0 {
                *count.get_mut(&(num / 2)).unwrap() -= 1;
                original.push(num / 2);
            } else {
                *count.entry(num).or_insert(0) += 1;
            }
        }

        if original.len() * 2 != changed.len() {
            original.clear();
        }

        original
    }
}
```
