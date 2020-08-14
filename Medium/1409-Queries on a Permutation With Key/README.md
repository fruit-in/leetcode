# 1409. Queries on a Permutation With Key
Given the array `queries` of positive integers between `1` and `m`, you have to process all `queries[i]` (from `i=0` to `i=queries.length-1`) according to the following rules:
* In the beginning, you have the permutation `P=[1,2,3,...,m]`.
* For the current `i`, find the position of `queries[i]` in the permutation `P` (**indexing from 0**) and then move this at the beginning of the permutation `P`. Notice that the position of `queries[i]` in `P` is the result for `queries[i]`.

Return an array containing the result for the given `queries`.

#### Example 1:
<pre>
<strong>Input:</strong> queries = [3,1,2,1], m = 5
<strong>Output:</strong> [2,1,2,1]
<strong>Explanation:</strong> The queries are processed as follow:
For i=0: queries[i]=3, P=[1,2,3,4,5], position of 3 in P is <b>2</b>, then we move 3 to the beginning of P resulting in P=[3,1,2,4,5].
For i=1: queries[i]=1, P=[3,1,2,4,5], position of 1 in P is <b>1</b>, then we move 1 to the beginning of P resulting in P=[1,3,2,4,5].
For i=2: queries[i]=2, P=[1,3,2,4,5], position of 2 in P is <b>2</b>, then we move 2 to the beginning of P resulting in P=[2,1,3,4,5].
For i=3: queries[i]=1, P=[2,1,3,4,5], position of 1 in P is <b>1</b>, then we move 1 to the beginning of P resulting in P=[1,2,3,4,5].
Therefore, the array containing the result is [2,1,2,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> queries = [4,1,2,2], m = 4
<strong>Output:</strong> [3,1,2,0]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> queries = [7,5,5,8,3], m = 8
<strong>Output:</strong> [6,5,0,7,5]
</pre>

#### Constraints:
* `1 <= m <= 10^3`
* `1 <= queries.length <= m`
* `1 <= queries[i] <= m`

## Solutions (Ruby)

### 1. Simulation
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

## Solutions (Rust)

### 1. Simulation
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

## Solutions (Kotlin)

### 1. Simulation
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
