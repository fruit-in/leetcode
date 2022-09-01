# 1053. 交换一次的先前排列
给你一个正整数的数组 `A`（其中的元素不一定完全不同），请你返回可在 **一次交换**（交换两数字 `A[i]` 和 `A[j]` 的位置）后得到的、按字典序排列小于 `A` 的最大可能排列。

如果无法这么操作，就请返回原数组。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,2,1]
<strong>输出:</strong> [3,1,2]
<strong>解释:</strong> 交换 2 和 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,5]
<strong>输出:</strong> [1,1,5]
<strong>解释:</strong> 已经是最小排列
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,9,4,6,7]
<strong>输出:</strong> [1,7,4,6,9]
<strong>解释:</strong> 交换 9 和 7
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [3,1,1,3]
<strong>输出:</strong> [1,3,1,3]
<strong>解释:</strong> 交换 1 和 3
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* <code>1 <= arr[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        for i in (0..arr.len() - 1).rev() {
            if arr[i] > arr[i + 1] {
                let mut j = i + 1;

                for k in i + 2..arr.len() {
                    if arr[j] < arr[k] && arr[i] > arr[k] {
                        j = k;
                    }
                }

                arr.swap(i, j);

                break;
            }
        }

        arr
    }
}
```
