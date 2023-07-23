# 1640. 能否连接形成数组
给你一个整数数组 `arr` ，数组中的每个整数 **互不相同** 。另有一个由整数数组构成的数组 `pieces`，其中的整数也 **互不相同** 。请你以 **任意顺序** 连接 `pieces` 中的数组以形成 `arr` 。但是，**不允许** 对每个数组 `pieces[i]` 中的整数重新排序。

如果可以连接 `pieces` 中的数组形成 `arr` ，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [85], pieces = [[85]]
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [15,88], pieces = [[88],[15]]
<strong>输出:</strong> true
<strong>解释:</strong> 依次连接 [15] 和 [88]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [49,18,16], pieces = [[16,18,49]]
<strong>输出:</strong> false
<strong>解释:</strong> 即便数字相符，也不能重新排列 pieces[0]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [91,4,64,78], pieces = [[78],[4,64],[91]]
<strong>输出:</strong> true
<strong>解释:</strong> 依次连接 [91]、[4,64] 和 [78]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [1,3,5,7], pieces = [[2,4,6,8]]
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= pieces.length <= arr.length <= 100`
* `sum(pieces[i].length) == arr.length`
* `1 <= pieces[i].length <= arr.length`
* `1 <= arr[i], pieces[i][j] <= 100`
* `arr` 中的整数 **互不相同**
* `pieces` 中的整数 **互不相同**（也就是说，如果将 `pieces` 扁平化成一维数组，数组中的所有整数互不相同）

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} arr
# @param {Integer[][]} pieces
# @return {Boolean}
def can_form_array(arr, pieces)
  indices = arr.map.with_index.to_h
  indices.default = 0

  arr == pieces.sort_by { |piece| indices[piece[0]] }.flatten(1)
end
```

## 题解 (Rust)

### 1. 排序
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
        let indices = arr
            .iter()
            .enumerate()
            .map(|(i, n)| (n, i))
            .collect::<HashMap<_, _>>();

        pieces.sort_unstable_by_key(|piece| indices.get(&piece[0]));

        arr == pieces.concat()
    }
}
```
