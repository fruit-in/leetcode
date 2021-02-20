# 658. 找到 K 个最接近的元素
给定一个排序好的数组 `arr` ，两个整数 `k` 和 `x` ，从数组中找到最靠近 `x`（两数之差最小）的 `k` 个数。返回的结果必须要是按升序排好的。

整数 `a` 比整数 `b` 更接近 `x` 需要满足：
* `|a - x| < |b - x|` 或者
* `|a - x| == |b - x|` 且 `a < b`

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5], k = 4, x = 3
<strong>输出:</strong> [1,2,3,4]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5], k = 4, x = -1
<strong>输出:</strong> [1,2,3,4]
</pre>

#### 提示:
* `1 <= k <= arr.length`
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* 数组里的每个元素与 `x` 的绝对值不超过 <code>10<sup>4</sup></code>

## 题解 (Rust)

### 1. 二分查找和双指针
```Rust
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut l = match arr.binary_search(&x) {
            Err(i) if i == arr.len() => return arr.split_at(i - k as usize).1.to_vec(),
            Err(i) if i == 0 => return arr[0..k as usize].to_vec(),
            Err(i) if (arr[i - 1] - x).abs() <= arr[i] - x => i - 1,
            Ok(i) | Err(i) => i,
        };
        let mut r = l + 1;

        while ((r - l) as i32) < k {
            if r == arr.len() || (l > 0 && (arr[l - 1] - x).abs() <= arr[r] - x) {
                l -= 1;
            } else {
                r += 1;
            }
        }

        arr[l..r].to_vec()
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        arr.sort_unstable_by_key(|&y| ((y - x).abs(), y));
        arr.truncate(k as usize);
        arr.sort_unstable();

        arr
    }
}
```
