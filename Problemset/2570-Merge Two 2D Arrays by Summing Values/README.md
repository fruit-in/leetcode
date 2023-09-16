# 2570. Merge Two 2D Arrays by Summing Values
You are given two **2D** integer arrays `nums1` and `nums2`.

* <code>nums1[i] = [id<sub>i</sub>, val<sub>i</sub>]</code> indicate that the number with the id <code>id<sub>i</sub></code> has a value equal to <code>val<sub>i</sub></code>.
* <code>nums2[i] = [id<sub>i</sub>, val<sub>i</sub>]</code> indicate that the number with the id <code>id<sub>i</sub></code> has a value equal to <code>val<sub>i</sub></code>.

Each array contains **unique** ids and is sorted in **ascending** order by id.

Merge the two arrays into one array that is sorted in ascending order by id, respecting the following conditions:

* Only ids that appear in at least one of the two arrays should be included in the resulting array.
* Each id should be included **only once** and its value should be the sum of the values of this id in the two arrays. If the id does not exist in one of the two arrays then its value in that array is considered to be `0`.

Return *the resulting array*. The returned array must be sorted in ascending order by id.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [[1,2],[2,3],[4,5]], nums2 = [[1,4],[3,2],[4,1]]
<strong>Output:</strong> [[1,6],[2,3],[3,2],[4,6]]
<strong>Explanation:</strong> The resulting array contains the following:
- id = 1, the value of this id is 2 + 4 = 6.
- id = 2, the value of this id is 3.
- id = 3, the value of this id is 2.
- id = 4, the value of this id is 5 + 1 = 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [[2,4],[3,6],[5,5]], nums2 = [[1,3],[4,3]]
<strong>Output:</strong> [[1,3],[2,4],[3,6],[4,3],[5,5]]
<strong>Explanation:</strong> There are no common ids, so we just include each id with its value in the resulting list.
</pre>

#### Constraints:
* `1 <= nums1.length, nums2.length <= 200`
* `nums1[i].length == nums2[j].length == 2`
* <code>1 <= id<sub>i</sub>, val<sub>i</sub> <= 1000</code>
* Both arrays contain unique ids.
* Both arrays are in strictly ascending order by id.

## Solutions (Rust)

### 1. Solution
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
