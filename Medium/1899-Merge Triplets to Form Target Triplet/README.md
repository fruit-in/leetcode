# 1899. Merge Triplets to Form Target Triplet
A **triplet** is an array of three integers. You are given a 2D integer array `triplets`, where <code>triplets[i] = [a<sub>i</sub>, b<sub>i</sub>, c<sub>i</sub>]</code> describes the <code>i<sup>th</sup></code> **triplet**. You are also given an integer array `target = [x, y, z]` that describes the **triplet** you want to obtain.

To obtain `target`, you may apply the following operation on `triplets` **any number** of times (possibly **zero**):

* Choose two indices (**0-indexed**) `i` and `j` (`i != j`) and **update** `triplets[j]` to become <code>[max(a<sub>i</sub>, a<sub>j</sub>), max(b<sub>i</sub>, b<sub>j</sub>), max(c<sub>i</sub>, c<sub>j</sub>)]</code>.
  * For example, if `triplets[i] = [2, 5, 3]` and `triplets[j] = [1, 7, 5]`, `triplets[j]` will be updated to `[max(2, 1), max(5, 7), max(3, 5)] = [2, 7, 5]`.

Return `true` *if it is possible to obtain the* `target` ***triplet*** `[x, y, z]` *as an **element** of* `triplets`, *or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> Perform the following operations:
- Choose the first and last triplets [[2,5,3],[1,8,4],[1,7,5]]. Update the last triplet to be [max(2,1), max(5,7), max(3,5)] = [2,7,5]. triplets = [[2,5,3],[1,8,4],[2,7,5]]
The target triplet [2,7,5] is now an element of triplets.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> triplets = [[3,4,5],[4,5,6]], target = [3,2,5]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to have [3,2,5] as an element because there is no 2 in any of the triplets.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> triplets = [[2,5,3],[2,3,4],[1,2,5],[5,2,3]], target = [5,5,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> Perform the following operations:
- Choose the first and third triplets [[2,5,3],[2,3,4],[1,2,5],[5,2,3]]. Update the third triplet to be [max(2,1), max(5,2), max(3,5)] = [2,5,5]. triplets = [[2,5,3],[2,3,4],[2,5,5],[5,2,3]].
- Choose the third and fourth triplets [[2,5,3],[2,3,4],[2,5,5],[5,2,3]]. Update the fourth triplet to be [max(2,5), max(5,2), max(5,3)] = [5,5,5]. triplets = [[2,5,3],[2,3,4],[2,5,5],[5,5,5]].
The target triplet [5,5,5] is now an element of triplets.
</pre>

#### Constraints:
* <code>1 <= triplets.length <= 10<sup>5</sup></code>
* `triplets[i].length == target.length == 3`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub>, c<sub>i</sub>, x, y, z <= 1000</code>

## Solutions (Rust)

### 1. Solution
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
