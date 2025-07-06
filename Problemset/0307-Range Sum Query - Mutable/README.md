# 307. Range Sum Query - Mutable
Given an integer array `nums`, handle multiple queries of the following types:
1. **Update** the value of an element in `nums`.
2. Calculate the **sum** of the elements of `nums` between indices `left` and `right` **inclusive** where `left <= right`.

Implement the `NumArray` class:
* `NumArray(int[] nums)` Initializes the object with the integer array `nums`.
* `void update(int index, int val)` **Updates** the value of `nums[index]` to be `val`.
* `int sumRange(int left, int right)` Returns the **sum** of the elements of `nums` between indices `left` and `right` **inclusive** (i.e. `nums[left] + nums[left + 1] + ... + nums[right]`).

#### Example 1:
<pre>
<strong>Input:</strong>
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
<strong>Output:</strong>
[null, 9, null, 8]
<strong>Explanation:</strong>
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // return 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1, 2, 5]
numArray.sumRange(0, 2); // return 1 + 2 + 5 = 8
</pre>

#### Constraints:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* `-100 <= nums[i] <= 100`
* `0 <= index < nums.length`
* `-100 <= val <= 100`
* `0 <= left <= right < nums.length`
* At most <code>3 * 10<sup>4</sup></code> calls will be made to `update` and `sumRange`.

## Solutions (Python)

### 1. Solution
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
