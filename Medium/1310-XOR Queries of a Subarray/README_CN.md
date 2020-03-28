# 1310. 子数组异或查询
有一个正整数数组 ```arr```，现给你一个对应的查询数组 ```queries```，其中 <code>queries[i] = [L<sub>i</sub>, R<sub>i</sub>]</code>。

对于每个查询 ```i```，请你计算从 <code>L<sub>i</sub></code> 到 <code>R<sub>i</sub></code> 的 **XOR** 值（即 <code>arr[L<sub>i</sub>] <strong>xor</strong> arr[L<sub>i</sub>+1] <strong>xor</strong> ... <strong>xor</strong> arr[R<sub>i</sub>]</code>）作为本次查询的结果。

并返回一个包含给定查询 ```queries``` 所有结果的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
<strong>输出:</strong> [2,7,14,8]
<strong>解释:</strong>
数组中元素的二进制表示形式是：
1 = 0001
3 = 0011
4 = 0100
8 = 1000
查询的 XOR 值为：
[0,1] = 1 xor 3 = 2
[1,2] = 3 xor 4 = 7
[0,3] = 1 xor 3 xor 4 xor 8 = 14
[3,3] = 8
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [4,8,2,10], queries = [[2,3],[1,3],[0,0],[0,3]]
<strong>输出:</strong> [8,0,4,4]
</pre>

#### 提示:
* ```1 <= arr.length <= 3 * 10^4```
* ```1 <= arr[i] <= 10^9```
* ```1 <= queries.length <= 3 * 10^4```
* ```queries[i].length == 2```
* ```0 <= queries[i][0] <= queries[i][1] < arr.length```

## 题解 (Rust)

### 1. 前缀和
```Rust
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = arr;
        let mut ret = Vec::new();

        for i in 1..arr.len() {
            arr[i] ^= arr[i - 1];
        }

        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize;
            ret.push(arr.get(l - 1).unwrap_or(&0) ^ arr[r]);
        }

        ret
    }
}
```
