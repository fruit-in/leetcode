# 2570. 合并两个二维数组 - 求和法
给你两个 **二维** 整数数组 `nums1` 和 `nums2`.

* <code>nums1[i] = [id<sub>i</sub>, val<sub>i</sub>]</code> 表示编号为 <code>id<sub>i</sub></code> 的数字对应的值等于 <code>val<sub>i</sub></code> 。
* <code>nums2[i] = [id<sub>i</sub>, val<sub>i</sub>]</code> 表示编号为 <code>id<sub>i</sub></code> 的数字对应的值等于 <code>val<sub>i</sub></code> 。

每个数组都包含 **互不相同** 的 id ，并按 id 以 **递增** 顺序排列。

请你将两个数组合并为一个按 id 以递增顺序排列的数组，并符合下述条件：

* 只有在两个数组中至少出现过一次的 id 才能包含在结果数组内。
* 每个 id 在结果数组中 **只能出现一次** ，并且其对应的值等于两个数组中该 id 所对应的值求和。如果某个数组中不存在该 id ，则认为其对应的值等于 `0` 。

返回结果数组。返回的数组需要按 id 以递增顺序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [[1,2],[2,3],[4,5]], nums2 = [[1,4],[3,2],[4,1]]
<strong>输出:</strong> [[1,6],[2,3],[3,2],[4,6]]
<strong>解释:</strong> 结果数组中包含以下元素：
- id = 1 ，对应的值等于 2 + 4 = 6 。
- id = 2 ，对应的值等于 3 。
- id = 3 ，对应的值等于 2 。
- id = 4 ，对应的值等于5 + 1 = 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [[2,4],[3,6],[5,5]], nums2 = [[1,3],[4,3]]
<strong>输出:</strong> [[1,3],[2,4],[3,6],[4,3],[5,5]]
<strong>解释:</strong> 不存在共同 id ，在结果数组中只需要包含每个 id 和其对应的值。
</pre>

#### 提示:
* `1 <= nums1.length, nums2.length <= 200`
* `nums1[i].length == nums2[j].length == 2`
* <code>1 <= id<sub>i</sub>, val<sub>i</sub> <= 1000</code>
* 数组中的 id 互不相同
* 数据均按 id 以严格递增顺序排列

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ret = vec![];

        while i < nums1.len() || j < nums2.len() {
            if j >= nums2.len() {
                ret.push(nums1[i].clone());
                i += 1;
            } else if i >= nums1.len() {
                ret.push(nums2[j].clone());
                j += 1;
            } else if nums1[i][0] == nums2[j][0] {
                ret.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            } else if nums1[i][0] < nums2[j][0] {
                ret.push(nums1[i].clone());
                i += 1;
            } else {
                ret.push(nums2[j].clone());
                j += 1;
            }
        }

        ret
    }
}
```
