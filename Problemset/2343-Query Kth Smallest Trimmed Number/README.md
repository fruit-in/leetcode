# 2343. Query Kth Smallest Trimmed Number
You are given a **0-indexed** array of strings `nums`, where each string is of **equal length** and consists of only digits.

You are also given a **0-indexed** 2D integer array `queries` where <code>queries[i] = [k<sub>i</sub>, trim<sub>i</sub>]</code>. For each `queries[i]`, you need to:

* **Trim** each number in `nums` to its **rightmost** <code>trim<sub>i</sub></code> digits.
* Determine the **index** of the <code>k<sub>i</sub><sup>th</sup></code> smallest trimmed number in `nums`. If two trimmed numbers are equal, the number with the **lower** index is considered to be smaller.
* Reset each number in `nums` to its original length.

Return *an array* `answer` *of the same length as* `queries`, *where* `answer[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*.

**Note**:

* To trim to the rightmost `x` digits means to keep removing the leftmost digit, until only `x` digits remain.
* Strings in `nums` may contain leading zeros.

#### Example 1:
<pre>
<strong>Input:</strong> nums = ["102","473","251","814"], queries = [[1,1],[2,3],[4,2],[1,2]]
<strong>Output:</strong> [2,2,1,0]
<strong>Explanation:</strong>
1. After trimming to the last digit, nums = ["2","3","1","4"]. The smallest number is 1 at index 2.
2. Trimmed to the last 3 digits, nums is unchanged. The 2nd smallest number is 251 at index 2.
3. Trimmed to the last 2 digits, nums = ["02","73","51","14"]. The 4th smallest number is 73.
4. Trimmed to the last 2 digits, the smallest number is 2 at index 0.
   Note that the trimmed number "02" is evaluated as 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = ["24","37","96","04"], queries = [[2,1],[2,2]]
<strong>Output:</strong> [3,0]
<strong>Explanation:</strong>
1. Trimmed to the last digit, nums = ["4","7","6","4"]. The 2nd smallest number is 4 at index 3.
   There are two occurrences of 4, but the one at index 0 is considered smaller than the one at index 3.
2. Trimmed to the last 2 digits, nums is unchanged. The 2nd smallest number is 24.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i].length <= 100`
* `nums[i]` consists of only digits.
* All `nums[i].length` are **equal**.
* `1 <= queries.length <= 100`
* `queries[i].length == 2`
* <code>1 <= k<sub>i</sub> <= nums.length</code>
* <code>1 <= trim<sub>i</sub> <= nums[i].length</code>

**Follow up:** Could you use the **Radix Sort Algorithm** to solve this problem? What will be the complexity of that solution?

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def smallestTrimmedNumbers(self, nums: List[str], queries: List[List[int]]) -> List[int]:
        nums = [(nums[i], i) for i in range(len(nums))]
        trims = {}
        answer = [0] * len(queries)

        for i in range(len(queries)):
            if queries[i][1] not in trims:
                trims[queries[i][1]] = []
            trims[queries[i][1]].append((i, queries[i][0] - 1))

        for trim in range(1, len(nums[0][0]) + 1):
            count = [0] * 10
            tmp = [("", 0)] * len(nums)

            for num, _ in nums:
                count[ord(num[-trim]) - 48] += 1
            for i in range(1, 10):
                count[i] += count[i - 1]
            for num, i in nums[::-1]:
                count[ord(num[-trim]) - 48] -= 1
                tmp[count[ord(num[-trim]) - 48]] = (num, i)

            for i, k in trims.get(trim, []):
                answer[i] = tmp[k][1]

            nums = tmp

        return answer
```
