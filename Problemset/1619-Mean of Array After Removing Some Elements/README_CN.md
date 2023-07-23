# 1619. 删除某些元素后的数组均值
给你一个整数数组 `arr` ，请你删除最小 `5%` 的数字和最大 `5%` 的数字后，剩余数字的平均值。

与 **标准答案** 误差在 <code>10<sup>-5</sup></code> 的结果都被视为正确结果。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]
<strong>输出:</strong> 2.00000
<strong>解释:</strong> 删除数组中最大和最小的元素后，所有元素都等于 2，所以平均值为 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]
<strong>输出:</strong> 4.00000
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]
<strong>输出:</strong> 4.77778
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [9,7,8,7,7,8,4,4,6,8,8,7,6,8,8,9,2,6,0,0,1,10,8,6,3,3,5,1,10,9,0,7,10,0,10,4,1,10,6,9,3,6,0,0,2,7,0,6,7,2,9,7,7,3,0,1,6,1,10,3]
<strong>输出:</strong> 5.27778
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [4,8,4,10,0,7,1,3,7,8,8,3,4,1,6,2,1,1,8,0,9,8,0,3,9,10,3,10,1,10,7,3,2,1,4,9,10,7,6,4,0,8,5,1,2,1,6,2,5,0,7,10,9,10,3,7,10,5,8,5,7,6,7,6,10,9,5,10,5,5,7,2,10,7,7,8,2,0,1,1]
<strong>输出:</strong> 5.29167
</pre>

#### 提示:
* `20 <= arr.length <= 1000`
* `arr.length` 是 `20` 的 **倍数**
* <code>0 <= arr[i] <= 10<sup>5</sup></code>

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} arr
# @return {Float}
def trim_mean(arr)
  length = arr.length

  arr.sort!
  arr = arr[length / 20...-length / 20]

  arr.sum / (length * 0.9)
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let len = arr.len() * 9 / 10;

        arr.sort_unstable();

        arr.iter().skip(len / 18).take(len).sum::<i32>() as f64 / (len as f64)
    }
}
```
