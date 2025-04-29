# 457. 环形数组是否存在循环
存在一个不含 `0` 的 **环形** 数组 `nums` ，每个 `nums[i]` 都表示位于下标 `i` 的角色应该向前或向后移动的下标个数：
* 如果 `nums[i]` 是正数，**向前**（下标递增方向）移动 `|nums[i]|` 步
* 如果 `nums[i]` 是负数，**向后**（下标递减方向）移动 `|nums[i]|` 步

因为数组是 **环形** 的，所以可以假设从最后一个元素向前移动一步会到达第一个元素，而第一个元素向后移动一步会到达最后一个元素。

数组中的 **循环** 由长度为 `k` 的下标序列 `seq` 标识：
* 遵循上述移动规则将导致一组重复下标序列 `seq[0] -> seq[1] -> ... -> seq[k - 1] -> seq[0] -> ...`
* 所有 `nums[seq[j]]` 应当不是 **全正** 就是 **全负**
* `k > 1`

如果 `nums` 中存在循环，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/09/01/img1.jpg)
<pre>
<strong>输入:</strong> nums = [2,-1,1,2,2]
<strong>输出:</strong> true
<strong>解释:</strong> 图片展示了节点间如何连接。白色节点向前跳跃，而红色节点向后跳跃。
我们可以看到存在循环，按下标 0 -> 2 -> 3 -> 0 --> ...，并且其中的所有节点都是白色（以相同方向跳跃）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/09/01/img2.jpg)
<pre>
<strong>输入:</strong> nums = [-1,-2,-3,-4,-5,6]
<strong>输出:</strong> false
<strong>解释:</strong> 图片展示了节点间如何连接。白色节点向前跳跃，而红色节点向后跳跃。
唯一的循环长度为 1，所以返回 false。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2022/09/01/img3.jpg)
<pre>
<strong>输入:</strong> nums = [1,-1,5,1,4]
<strong>输出:</strong> true
<strong>解释:</strong> 图片展示了节点间如何连接。白色节点向前跳跃，而红色节点向后跳跃。
我们可以看到存在循环，按下标 0 --> 1 --> 0 --> ...，当它的大小大于 1 时，它有一个向前跳的节点和一个向后跳的节点，所以 它不是一个循环。
我们可以看到存在循环，按下标 3 --> 4 --> 3 --> ...，并且其中的所有节点都是白色（以相同方向跳跃）。
</pre>

#### 提示:
* `1 <= nums.length <= 5000`
* `-1000 <= nums[i] <= 1000`
* `nums[i] != 0`

**进阶：**你能设计一个时间复杂度为 `O(n)` 且额外空间复杂度为 `O(1)` 的算法吗？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
        let n = nums.len();

        for i in 0..nums.len() {
            nums[i] %= n as i32;
        }

        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }

            let positive = nums[i] > 0;
            let mut j = i;

            while nums[j] != 0 && (nums[j] > 0) == positive {
                if nums[j].abs() > n as i32 {
                    return true;
                }

                if positive {
                    nums[j] += n as i32;
                    j = (j + nums[j] as usize) % n;
                } else {
                    nums[j] -= n as i32;
                    j = (j + n * 2 - nums[j].abs() as usize) % n;
                }
            }

            j = i;

            while nums[j] != 0 && (nums[j] > 0) == positive {
                let tmp = j;
                if positive {
                    j = (j + nums[j] as usize) % n;
                    nums[tmp] = 0;
                } else {
                    j = (j + n * 2 - nums[j].abs() as usize) % n;
                    nums[tmp] = 0;
                }
            }
        }

        false
    }
}
```
