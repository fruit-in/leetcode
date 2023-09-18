# 406. 根据身高重建队列
假设有打乱顺序的一群人站成一个队列，数组 `people` 表示队列中一些人的属性（不一定按顺序）。每个 <code>people[i] = [h<sub>i</sub>, k<sub>i</sub>]</code> 表示第 `i` 个人的身高为 <code>h<sub>i</sub></code> ，前面 正好 有 <code>k<sub>i</sub></code> 个身高大于或等于 <code>h<sub>i</sub></code> 的人。

请你重新构造并返回输入数组 `people` 所表示的队列。返回的队列应该格式化为数组 `queue` ，其中 <code>queue[j] = [h<sub>j</sub>, k<sub>j</sub>]</code> 是队列中第 `j` 个人的属性（`queue[0]` 是排在队列前面的人）。

#### 示例 1:
<pre>
<strong>输入:</strong> people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
<strong>输出:</strong> [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
<strong>解释:</strong>
编号为 0 的人身高为 5 ，没有身高更高或者相同的人排在他前面。
编号为 1 的人身高为 7 ，没有身高更高或者相同的人排在他前面。
编号为 2 的人身高为 5 ，有 2 个身高更高或者相同的人排在他前面，即编号为 0 和 1 的人。
编号为 3 的人身高为 6 ，有 1 个身高更高或者相同的人排在他前面，即编号为 1 的人。
编号为 4 的人身高为 4 ，有 4 个身高更高或者相同的人排在他前面，即编号为 0、1、2、3 的人。
编号为 5 的人身高为 7 ，有 1 个身高更高或者相同的人排在他前面，即编号为 1 的人。
因此 [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] 是重新构造后的队列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
<strong>输出:</strong> [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]
</pre>

#### 提示:
* `1 <= people.length <= 2000`
* <code>0 <= h<sub>i</sub> <= 10<sup>6</sup></code>
* <code>0 <= k<sub>i</sub> < people.length</code>
* 题目数据确保队列可以被重建

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let mut queue = vec![];

        people.sort_unstable_by_key(|p| (-p[0], p[1]));

        for p in people {
            queue.insert(p[1] as usize, p);
        }

        queue
    }
}
```
