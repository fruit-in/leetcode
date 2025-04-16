# 1889. Minimum Space Wasted From Packaging
You have `n` packages that you are trying to place in boxes, **one package in each box**. There are `m` suppliers that each produce boxes of **different sizes** (with infinite supply). A package can be placed in a box if the size of the package is **less than or equal to** the size of the box.

The package sizes are given as an integer array `packages`, where `packages[i]` is the **size** of the <code>i<sup>th</sup></code> package. The suppliers are given as a 2D integer array `boxes`, where `boxes[j]` is an array of **box sizes** that the <code>j<sup>th</sup></code> supplier produces.

You want to choose a **single supplier** and use boxes from them such that the **total wasted space** is **minimized**. For each package in a box, we define the space **wasted** to be `size of the box - size of the package`. The **total wasted space** is the sum of the space wasted in **all** the boxes.

* For example, if you have to fit packages with sizes `[2,3,5]` and the supplier offers boxes of sizes `[4,8]`, you can fit the packages of size-`2` and size-`3` into two boxes of size-`4` and the package with size-`5` into a box of size-`8`. This would result in a waste of `(4-2) + (4-3) + (8-5) = 6`.

Return *the **minimum total wasted space** by choosing the box supplier **optimally**, or* `-1` *if it is **impossible** to fit all the packages inside boxes*. Since the answer may be **large**, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> packages = [2,3,5], boxes = [[4,8],[2,8]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> It is optimal to choose the first supplier, using two size-4 boxes and one size-8 box.
The total waste is (4-2) + (4-3) + (8-5) = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> packages = [2,3,5], boxes = [[1,4],[2,3],[3,4]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no box that the package of size 5 can fit in.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> packages = [3,5,8,10,11,12], boxes = [[12],[11,9],[10,5,14]]
<strong>Output:</strong> 9
<strong>Explanation:</strong> It is optimal to choose the third supplier, using two size-5 boxes, two size-10 boxes, and two size-14 boxes.
The total waste is (5-3) + (5-5) + (10-8) + (10-10) + (14-11) + (14-12) = 9.
</pre>

#### Constraints:
* `n == packages.length`
* `m == boxes.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= m <= 10<sup>5</sup></code>
* <code>1 <= packages[i] <= 10<sup>5</sup></code>
* <code>1 <= boxes[j].length <= 10<sup>5</sup></code>
* <code>1 <= boxes[j][k] <= 10<sup>5</sup></code>
* <code>sum(boxes[j].length) <= 10<sup>5</sup></code>
* The elements in `boxes[j]` are **distinct**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minWastedSpace(self, packages: List[int], boxes: List[List[int]]) -> int:
        prefixsum = [0] * (len(packages) + 1)
        ret = float("inf")

        packages.sort()

        for i in range(len(packages)):
            prefixsum[i + 1] = prefixsum[i] + packages[i]

        for supplier in boxes:
            wasted = 0
            i = 0
            supplier.sort()

            if supplier[-1] < packages[-1]:
                continue

            for size in supplier:
                j = bisect.bisect(packages, size, lo=i)
                wasted += (j - i) * size - prefixsum[j] + prefixsum[i]
                i = j

            ret = min(ret, wasted)

        return -1 if ret == float("inf") else ret % 1000000007
```
