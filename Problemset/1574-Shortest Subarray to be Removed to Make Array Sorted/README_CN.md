# 1574. 删除最短的子数组使剩余数组有序
给你一个整数数组 `arr` ，请你删除一个子数组（可以为空），使得 `arr` 中剩下的元素是 **非递减** 的。

一个子数组指的是原数组中连续的一个子序列。

请你返回满足题目要求的最短子数组的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,3,10,4,2,3,5]
<strong>输出:</strong> 3
<strong>解释:</strong> 我们需要删除的最短子数组是 [10,4,2] ，长度为 3 。剩余元素形成非递减数组 [1,2,3,3,5] 。
另一个正确的解为删除子数组 [3,10,4] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [5,4,3,2,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 由于数组是严格递减的，我们只能保留一个元素。所以我们需要删除长度为 4 的子数组，要么删除 [5,4,3,2]，要么删除 [4,3,2,1]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,2,3]
<strong>输出:</strong> 0
<strong>解释:</strong> 数组已经是非递减的了，我们不需要删除任何元素。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [1]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>0 <= arr[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        let mut i = arr.len();
        let mut ret = arr.len() - 1;

        arr.insert(0, 0);

        while i > 0 && arr[i - 1] <= arr[i] {
            i -= 1;
        }

        if i == 0 {
            return 0;
        }

        for j in 0..arr.len() {
            if j > 0 && arr[j - 1] > arr[j] {
                break;
            }

            while i < arr.len() && arr[i] < arr[j] {
                i += 1;
            }

            ret = ret.min(i - j - 1);
        }

        ret as i32
    }
}
```
