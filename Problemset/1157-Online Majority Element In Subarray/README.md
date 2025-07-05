# 1157. Online Majority Element In Subarray
Design a data structure that efficiently finds the **majority element** of a given subarray.

The **majority element** of a subarray is an element that occurs `threshold` times or more in the subarray.

Implementing the `MajorityChecker` class:
* `MajorityChecker(int[] arr)` Initializes the instance of the class with the given array `arr`.
* `int query(int left, int right, int threshold)` returns the element in the subarray `arr[left...right]` that occurs at least `threshold` times, or `-1` if no such element exists.

#### Example 1:
<pre>
<strong>Input:</strong>
["MajorityChecker", "query", "query", "query"]
[[[1, 1, 2, 2, 1, 1]], [0, 5, 4], [0, 3, 3], [2, 3, 2]]
<strong>Output:</strong>
[null, 1, -1, 2]
<strong>Explanation:</strong>
MajorityChecker majorityChecker = new MajorityChecker([1, 1, 2, 2, 1, 1]);
majorityChecker.query(0, 5, 4); // return 1
majorityChecker.query(0, 3, 3); // return -1
majorityChecker.query(2, 3, 2); // return 2
</pre>

#### Constraints:
* <code>1 <= arr.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= arr[i] <= 2 * 10<sup>4</sup></code>
* `0 <= left <= right < arr.length`
* `threshold <= right - left + 1`
* `2 * threshold > right - left + 1`
* At most <code>10<sup>4</sup></code> calls will be made to `query`.

## Solutions (Python)

### 1. Solution
```Python
class MajorityChecker:

    def __init__(self, arr: List[int]):
        self.arr = arr
        self.indices = {}

        for i, x in enumerate(arr):
            if x not in self.indices:
                self.indices[x] = []
            self.indices[x].append(i)

    def query(self, left: int, right: int, threshold: int) -> int:
        used = set()
        count = 0

        while right - left + 1 - count >= threshold:
            x = self.arr[randint(left, right)]
            while x in used:
                x = self.arr[randint(left, right)]
            used.add(x)
            indices = self.indices[x]
            occur = bisect_right(indices, right) - bisect_left(indices, left)
            count += occur
            if occur >= threshold:
                return x

        return -1


# Your MajorityChecker object will be instantiated and called as such:
# obj = MajorityChecker(arr)
# param_1 = obj.query(left,right,threshold)
```
