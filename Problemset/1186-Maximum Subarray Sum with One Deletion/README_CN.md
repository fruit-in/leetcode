# 1186. 删除一次得到子数组最大和
给你一个整数数组，返回它的某个 **非空** 子数组（连续元素）在执行一次可选的删除操作后，所能得到的最大元素总和。换句话说，你可以从原数组中选出一个子数组，并可以决定要不要从中删除一个元素（只能删一次哦），（删除后）子数组中至少应当有一个元素，然后该子数组（剩下）的元素总和是所有子数组之中最大的。

注意，删除一个元素后，子数组 **不能为空**。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,-2,0,3]
<strong>输出:</strong> 4
<strong>解释:</strong> 我们可以选出 [1, -2, 0, 3]，然后删掉 -2，这样得到 [1, 0, 3]，和最大。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,-2,-2,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 我们直接选出 [3]，这就是最大和。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [-1,-1,-1,-1]
<strong>输出:</strong> -1
<strong>解释:</strong> 最后得到的子数组不能为空，所以我们不能选择 [-1] 并从中删去 -1 来得到 0。
     我们应该直接选择 [-1]，或者选择 [-1, -1] 再从中删去一个 -1。
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= arr[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let (mut x, mut y) = (arr[0], 0);
        let mut ret = x;

        for i in 1..arr.len() {
            (x, y) = (arr[i].max(x + arr[i]), x.max(y + arr[i]));
            ret = ret.max(x).max(y);
        }

        ret
    }
}
```
