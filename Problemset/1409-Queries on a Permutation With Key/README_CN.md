# 1409. 查询带键的排列
给你一个待查数组 `queries` ，数组中的元素为 `1` 到 `m` 之间的正整数。 请你根据以下规则处理所有待查项 `queries[i]`（从 `i=0` 到 `i=queries.length-1`）：
* 一开始，排列 `P=[1,2,3,...,m]`。
* 对于当前的 `i` ，请你找出待查项 `queries[i]` 在排列 `P` 中的位置（**下标从 0 开始**），然后将其从原位置移动到排列 `P` 的起始位置（即下标为 0 处）。注意， `queries[i]` 在 `P` 中的位置就是 `queries[i]` 的查询结果。

请你以数组形式返回待查数组  `queries` 的查询结果。

#### 示例 1:
<pre>
<strong>输入:</strong> queries = [3,1,2,1], m = 5
<strong>输出:</strong> [2,1,2,1]
<strong>解释:</strong> 待查数组 queries 处理如下：
对于 i=0: queries[i]=3, P=[1,2,3,4,5], 3 在 P 中的位置是 <b>2</b>，接着我们把 3 移动到 P 的起始位置，得到 P=[3,1,2,4,5] 。
对于 i=1: queries[i]=1, P=[3,1,2,4,5], 1 在 P 中的位置是 <b>1</b>，接着我们把 1 移动到 P 的起始位置，得到 P=[1,3,2,4,5] 。
对于 i=2: queries[i]=2, P=[1,3,2,4,5], 2 在 P 中的位置是 <b>2</b>，接着我们把 2 移动到 P 的起始位置，得到 P=[2,1,3,4,5] 。
对于 i=3: queries[i]=1, P=[2,1,3,4,5], 1 在 P 中的位置是 <b>1</b>，接着我们把 1 移动到 P 的起始位置，得到 P=[1,2,3,4,5] 。
因此，返回的结果数组为 [2,1,2,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> queries = [4,1,2,2], m = 4
<strong>输出:</strong> [3,1,2,0]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> queries = [7,5,5,8,3], m = 8
<strong>输出:</strong> [6,5,0,7,5]
</pre>

#### 提示:
* `1 <= m <= 10^3`
* `1 <= queries.length <= m`
* `1 <= queries[i] <= m`

## 题解 (Ruby)

### 1. 模拟
```Ruby
# @param {Integer[]} queries
# @param {Integer} m
# @return {Integer[]}
def process_queries(queries, m)
    p = Array(m.downto(1))
    ret = Array.new(queries.length)

    for i in 0...queries.length
        ret[i] = m - 1 - p.index(queries[i])
        p.delete(queries[i])
        p.push(queries[i])
    end

    return ret
end
```

## 题解 (Rust)

### 1. 模拟
```Rust
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p = (1..=m).rev().collect::<Vec<i32>>();
        let mut ret = vec![0; queries.len()];

        for i in 0..queries.len() {
            let posi = p.iter().position(|&x| x == queries[i]).unwrap();
            let x = p.remove(posi);
            p.push(x);
            ret[i] = m - 1 - posi as i32;
        }

        ret
    }
}
```

## 题解 (Kotlin)

### 1. 模拟
```Kotlin
class Solution {
    fun processQueries(queries: IntArray, m: Int): IntArray {
        var p = ArrayList<Int>((1..m).toList().reversed())
        var ret = IntArray(queries.size)

        for (i in 0 until queries.size) {
            ret[i] = m - 1 - p.indexOf(queries[i])
            p.remove(queries[i])
            p.add(queries[i])
        }

        return ret
    }
}
```
