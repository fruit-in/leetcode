# 1282. 用户分组
有 ```n``` 位用户参加活动，他们的 **ID** 从 ```0``` 到 ```n - 1```，每位用户都 **恰好** 属于某一用户组。给你一个长度为 ```n``` 的数组 ```groupSizes```，其中包含每位用户所处的用户组的大小，请你返回用户分组情况（存在的用户组以及每个组中用户的 ID）。

你可以任何顺序返回解决方案，ID 的顺序也不受限制。此外，题目给出的数据保证至少存在一种解决方案。

#### 示例 1:
<pre>
<strong>输入:</strong> groupSizes = [3,3,3,3,3,1,3]
<strong>输出:</strong> [[5],[0,1,2],[3,4,6]]
<strong>解释:</strong>
其他可能的解决方案有 [[2,1,6],[5],[0,4,3]] 和 [[5],[0,6,2],[4,3,1]]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> groupSizes = [2,1,3,3,3,2]
<strong>输出:</strong> [[1],[0,5],[2,3,4]]
</pre>

#### 提示:
* ```groupSizes.length == n```
* ```1 <= n <= 500```
* ```1 <= groupSizes[i] <= n```

## 题解 (Rust)

### 1. 哈希表
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
