# 31. Next Permutation
A **permutation** of an array of integers is an arrangement of its members into a sequence or linear order.

* For example, for `arr = [1,2,3]`, the following are all the permutations of `arr`: `[1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1]`.

The **next permutation** of an array of integers is the next lexicographically greater permutation of its integer. More formally, if all the permutations of the array are sorted in one container according to their lexicographical order, then the **next permutation** of that array is the permutation that follows it in the sorted container. If such arrangement is not possible, the array must be rearranged as the lowest possible order (i.e., sorted in ascending order).

* For example, the next permutation of `arr = [1,2,3]` is `[1,3,2]`.
* Similarly, the next permutation of `arr = [2,3,1]` is `[3,1,2]`.
* While the next permutation of `arr = [3,2,1]` is `[1,2,3]` because `[3,2,1]` does not have a lexicographical larger rearrangement.

Given an array of integers `nums`, *find the next permutation of* `nums`.

The replacement must be **in place** and use only constant extra memory.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> [1,3,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,2,1]
<strong>Output:</strong> [1,2,3]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,5]
<strong>Output:</strong> [1,5,1]
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut indices = vec![];

        for i in (0..nums.len()).rev() {
            match indices.binary_search_by_key(&nums[i], |&j| nums[j]) {
                Ok(k) if k == indices.len() - 1 => indices[k] = i,
                Ok(k) => {
                    nums.swap(i, indices[k + 1]);
                    let mut tmp = nums.split_off(i + 1);
                    tmp.sort_unstable();
                    nums.append(&mut tmp);
                    return;
                }
                Err(k) if k == indices.len() => indices.push(i),
                Err(k) => {
                    nums.swap(i, indices[k]);
                    let mut tmp = nums.split_off(i + 1);
                    tmp.sort_unstable();
                    nums.append(&mut tmp);
                    return;
                }
            }
        }

        nums.reverse();
    }
}
```
