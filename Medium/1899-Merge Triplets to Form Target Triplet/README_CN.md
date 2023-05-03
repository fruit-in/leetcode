# 1899. 合并若干三元组以形成目标三元组
**三元组** 是一个由三个整数组成的数组。给你一个二维整数数组 `triplets` ，其中 <code>triplets[i] = [a<sub>i</sub>, b<sub>i</sub>, c<sub>i</sub>]</code> 表示第 `i` 个 三元组 。同时，给你一个整数数组 `target = [x, y, z]` ，表示你想要得到的 **三元组** 。

为了得到 `target` ，你需要对 `triplets` 执行下面的操作 **任意次**（可能 **零** 次）：

* 选出两个下标（下标 **从 0 开始** 计数）`i` 和 `j`（`i != j`），并 **更新** `triplets[j]` 为 <code>[max(a<sub>i</sub>, a<sub>j</sub>), max(b<sub>i</sub>, b<sub>j</sub>), max(c<sub>i</sub>, c<sub>j</sub>)]</code> 。
  * 例如，`triplets[i] = [2, 5, 3]` 且 `triplets[j] = [1, 7, 5]`，`triplets[j]` 将会更新为 `[max(2, 1), max(5, 7), max(3, 5)] = [2, 7, 5]` 。

如果通过以上操作我们可以使得目标 **三元组** `target` 成为 `triplets` 的一个 **元素** ，则返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]
<strong>输出:</strong> true
<strong>解释:</strong> 执行下述操作：
- 选择第一个和最后一个三元组 [[2,5,3],[1,8,4],[1,7,5]] 。更新最后一个三元组为 [max(2,1), max(5,7), max(3,5)] = [2,7,5] 。triplets = [[2,5,3],[1,8,4],[2,7,5]]
目标三元组 [2,7,5] 现在是 triplets 的一个元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> triplets = [[1,3,4],[2,5,8]], target = [2,5,8]
<strong>输出:</strong> true
<strong>解释:</strong> 目标三元组 [2,5,8] 已经是 triplets 的一个元素。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> triplets = [[2,5,3],[2,3,4],[1,2,5],[5,2,3]], target = [5,5,5]
<strong>输出:</strong> true
<strong>解释:</strong> 执行下述操作：
- 选择第一个和第三个三元组 [[2,5,3],[2,3,4],[1,2,5],[5,2,3]] 。更新第三个三元组为 [max(2,1), max(5,2), max(3,5)] = [2,5,5] 。triplets = [[2,5,3],[2,3,4],[2,5,5],[5,2,3]] 。
- 选择第三个和第四个三元组 [[2,5,3],[2,3,4],[2,5,5],[5,2,3]] 。更新第四个三元组为 [max(2,5), max(5,2), max(5,3)] = [5,5,5] 。triplets = [[2,5,3],[2,3,4],[2,5,5],[5,5,5]] 。
目标三元组 [5,5,5] 现在是 triplets 的一个元素。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> triplets = [[3,4,5],[4,5,6]], target = [3,2,5]
<strong>输出:</strong> false
<strong>解释:</strong> 无法得到 [3,2,5] ，因为 triplets 不含 2 。
</pre>

#### 提示:
* <code>1 <= triplets.length <= 10<sup>5</sup></code>
* `triplets[i].length == target.length == 3`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub>, c<sub>i</sub>, x, y, z <= 1000</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut triplet = vec![0; 3];

        for tri in &triplets {
            if tri[0] <= target[0] && tri[1] <= target[1] && tri[2] <= target[2] {
                triplet[0] = triplet[0].max(tri[0]);
                triplet[1] = triplet[1].max(tri[1]);
                triplet[2] = triplet[2].max(tri[2]);
            }
        }

        triplet == target
    }
}
```
