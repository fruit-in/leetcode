# 1200. 最小绝对差
给你个整数数组 ```arr```，其中每个元素都 **不相同**。

请你找到所有具有最小绝对差的元素对，并且按升序的顺序返回。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [4,2,1,3]
<strong>输出:</strong> [[1,2],[2,3],[3,4]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,3,6,10,15]
<strong>输出:</strong> [[1,3]]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [3,8,-10,23,19,-4,-14,27]
<strong>输出:</strong> [[-14,-10],[19,23],[23,27]]
</pre>

#### 提示:
* ```2 <= arr.length <= 10^5```
* ```-10^6 <= arr[i] <= 10^6```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();
        let mut min = std::i32::MAX;
        let mut ret = Vec::new();

        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] < min {
                min = arr[i] - arr[i - 1];
                ret.clear();
                ret.push(vec![arr[i - 1], arr[i]]);
            } else if arr[i] - arr[i - 1] == min {
                ret.push(vec![arr[i - 1], arr[i]]);
            }
        }

        ret
    }
}
```
