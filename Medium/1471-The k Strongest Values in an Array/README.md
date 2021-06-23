# 1471. The k Strongest Values in an Array
Given an array of integers `arr` and an integer `k`.

A value `arr[i]` is said to be stronger than a value `arr[j]` if `|arr[i] - m| > |arr[j] - m|` where `m` is the **median** of the array.
If `|arr[i] - m| == |arr[j] - m|`, then `arr[i]` is said to be stronger than `arr[j]` if `arr[i] > arr[j]`.

Return *a list of the strongest `k`* values in the array. return the answer **in any arbitrary order**.

**Median** is the middle value in an ordered integer list. More formally, if the length of the list is n, the median is the element in position `((n - 1) / 2)` in the sorted list **(0-indexed)**.
* For `arr = [6, -3, 7, 2, 11]`, `n = 5` and the median is obtained by sorting the array `arr = [-3, 2, 6, 7, 11]` and the median is `arr[m]` where `m = ((5 - 1) / 2) = 2`. The median is `6`.
* For `arr = [-7, 22, 17, 3]`, `n = 4` and the median is obtained by sorting the array `arr = [-7, 3, 17, 22]` and the median is `arr[m]` where `m = ((4 - 1) / 2) = 1`. The median is `3`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5], k = 2
<strong>Output:</strong> [5,1]
<strong>Explanation:</strong> Median is 3, the elements of the array sorted by the strongest are [5,1,4,2,3]. The strongest 2 elements are [5, 1]. [1, 5] is also accepted answer.
Please note that although |5 - 3| == |1 - 3| but 5 is stronger than 1 because 5 > 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,3,5,5], k = 2
<strong>Output:</strong> [5,5]
<strong>Explanation:</strong> Median is 3, the elements of the array sorted by the strongest are [5,5,1,1,3]. The strongest 2 elements are [5, 5].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [6,7,11,7,6,8], k = 5
<strong>Output:</strong> [11,8,6,6,7]
<strong>Explanation:</strong> Median is 7, the elements of the array sorted by the strongest are [11,8,6,6,7,7].
Any permutation of [11,8,6,6,7] is accepted.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [6,-3,7,2,11], k = 3
<strong>Output:</strong> [-3,11,2]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [-7,22,17,3], k = 2
<strong>Output:</strong> [22,17]
</pre>

#### Constraints:
* `1 <= arr.length <= 10^5`
* `-10^5 <= arr[i] <= 10^5`
* `1 <= k <= arr.length`

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer[]}
def get_strongest(arr, k)
  arr.sort!
  m = (arr.size - 1) / 2
  i = 0
  j = arr.size - 1
  ret = []

  while ret.size < k
    if (arr[i] - arr[m]).abs > (arr[j] - arr[m]).abs
      ret.push(arr[i])
      i += 1
    else
      ret.push(arr[j])
      j -= 1
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        arr.sort_unstable();
        let m = (arr.len() - 1) / 2;
        let mut i = 0;
        let mut j = arr.len() - 1;
        let mut ret = vec![];

        while ret.len() < k as usize {
            if (arr[i] - arr[m]).abs() > (arr[j] - arr[m]).abs() {
                ret.push(arr[i]);
                i += 1;
            } else {
                ret.push(arr[j]);
                j -= 1;
            }
        }

        ret
    }
}
```
