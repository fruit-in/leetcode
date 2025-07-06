# 307. 区域和检索 - 数组可修改
给你一个数组 `nums` ，请你完成两类查询。

1. 其中一类查询要求 **更新** 数组 `nums` 下标对应的值
2. 另一类查询要求返回数组 `nums` 中索引 `left` 和索引 `right` 之间（ **包含** ）的nums元素的 **和** ，其中 `left <= right`

实现 `NumArray` 类：
* `NumArray(int[] nums)` 用整数数组 `nums` 初始化对象
* `void update(int index, int val)` 将 `nums[index]` 的值 **更新** 为 `val`
* `int sumRange(int left, int right)` 返回数组 `nums` 中索引 `left` 和索引 `right` 之间（ **包含** ）的nums元素的 **和** （即，`nums[left] + nums[left + 1], ..., nums[right]`）

#### 示例 1:
<pre>
<strong>输入:</strong>
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
<strong>输出:</strong>
[null, 9, null, 8]
<strong>解释:</strong>
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // 返回 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1,2,5]
numArray.sumRange(0, 2); // 返回 1 + 2 + 5 = 8
</pre>

#### 提示:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* `-100 <= nums[i] <= 100`
* `0 <= index < nums.length`
* `-100 <= val <= 100`
* `0 <= left <= right < nums.length`
* 调用 `update` 和 `sumRange` 方法次数不大于 <code>3 * 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class NumArray:

    def __init__(self, nums: List[int]):
        self.tree = [0] * (len(nums) + 1)

        for i, x in enumerate(nums):
            self.update(i, x)

    def update(self, index: int, val: int) -> None:
        val -= self.sumRange(index, index)
        index += 1

        while index < len(self.tree):
            self.tree[index] += val
            index += index & -index

    def sumRange(self, left: int, right: int) -> int:
        right += 1
        ret = 0

        while right > 0:
            ret += self.tree[right]
            right -= right & -right
        while left > 0:
            ret -= self.tree[left]
            left -= left & -left

        return ret


# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# obj.update(index,val)
# param_2 = obj.sumRange(left,right)
```
