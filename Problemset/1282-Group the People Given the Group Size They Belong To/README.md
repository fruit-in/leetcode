# 1282. Group the People Given the Group Size They Belong To
There are ```n``` people whose **IDs** go from ```0``` to ```n - 1``` and each person belongs **exactly** to one group. Given the array ```groupSizes``` of length ```n``` telling the group size each person belongs to, return the groups there are and the people's **IDs** each group includes.

You can return any solution in any order and the same applies for IDs. Also, it is guaranteed that there exists at least one solution.

#### Example 1:
<pre>
<strong>Input:</strong> groupSizes = [3,3,3,3,3,1,3]
<strong>Output:</strong> [[5],[0,1,2],[3,4,6]]
<strong>Explanation:</strong>
Other possible solutions are [[2,1,6],[5],[0,4,3]] and [[5],[0,6,2],[4,3,1]].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> groupSizes = [2,1,3,3,3,2]
<strong>Output:</strong> [[1],[0,5],[2,3,4]]
</pre>

#### Constraints:
* ```groupSizes.length == n```
* ```1 <= n <= 500```
* ```1 <= groupSizes[i] <= n```

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut group_ids = HashMap::new();
        let mut groups = Vec::new();

        for (id, size) in group_sizes.iter().enumerate() {
            let v = group_ids.entry(size).or_insert(Vec::new());
            (*v).push(id as i32);

            if v.len() == *size as usize {
                groups.push(group_ids.remove(size).unwrap());
            }
        }

        groups
    }
}
```
