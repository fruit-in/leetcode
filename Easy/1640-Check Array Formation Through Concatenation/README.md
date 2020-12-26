# 1640. Check Array Formation Through Concatenation
You are given an array of **distinct** integers `arr` and an array of integer arrays `pieces`, where the integers in `pieces` are **distinct**. Your goal is to form `arr` by concatenating the arrays in `pieces` **in any order**. However, you are **not** allowed to reorder the integers in each array `pieces[i]`.

Return `true` *if it is possible to form the array* `arr` *from* `pieces`. Otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [85], pieces = [[85]]
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [15,88], pieces = [[88],[15]]
<strong>Output:</strong> true
<strong>Explanation:</strong> Concatenate [15] then [88]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [49,18,16], pieces = [[16,18,49]]
<strong>Output:</strong> false
<strong>Explanation:</strong> Even though the numbers match, we cannot reorder pieces[0].
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [91,4,64,78], pieces = [[78],[4,64],[91]]
<strong>Output:</strong> true
<strong>Explanation:</strong> Concatenate [91] then [4,64] then [78]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [1,3,5,7], pieces = [[2,4,6,8]]
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= pieces.length <= arr.length <= 100`
* `sum(pieces[i].length) == arr.length`
* `1 <= pieces[i].length <= arr.length`
* `1 <= arr[i], pieces[i][j] <= 100`
* The integers in `arr` are **distinct**.
* The integers in `pieces` are **distinct** (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).

## Solutions (Ruby)

### 1. Sort
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

## Solutions (Rust)

### 1. Sort
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
