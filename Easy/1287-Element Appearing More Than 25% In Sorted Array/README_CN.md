# 1287. 有序数组中出现次数超过25%的元素
给你一个非递减的 **有序** 整数数组，已知这个数组中恰好有一个整数，它的出现次数超过数组元素总数的 25%。

请你找到并返回这个整数

#### 示例:
<pre>
<strong>输入:</strong> arr = [1,2,2,6,6,6,6,7,10]
<strong>输出:</strong> 6
</pre>

#### 提示:
* ```1 <= arr.length <= 10^4```
* ```0 <= arr[i] <= 10^5```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut l = 0;

        while let Ok(r) = arr.binary_search(&arr[l]) {
            if r - l >= arr.len() / 4 {
                return arr[l];
            }

            l = r + 1;
        }

        -1
    }
}
```
