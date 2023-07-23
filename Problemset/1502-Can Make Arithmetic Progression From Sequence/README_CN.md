# 1502. 判断能否形成等差数列
给你一个数字数组 `arr` 。

如果一个数列中，任意相邻两项的差总等于同一个常数，那么这个数列就称为 **等差数列** 。

如果可以重新排列数组形成等差数列，请返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,5,1]
<strong>输出:</strong> true
<strong>解释:</strong> 对数组重新排序得到 [1,3,5] 或者 [5,3,1] ，任意相邻两项的差分别为 2 或 -2 ，可以形成等差数列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,4]
<strong>输出:</strong> false
<strong>解释:</strong> 无法通过重新排序得到等差数列。
</pre>

#### 提示:
* `2 <= arr.length <= 1000`
* `-10^6 <= arr[i] <= 10^6`

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} arr
# @return {Boolean}
def can_make_arithmetic_progression(arr)
    arr.sort!

    for i in 2...arr.length
        return false if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2]
    end

    return true
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2] {
                return false;
            }
        }

        true
    }
}
```
