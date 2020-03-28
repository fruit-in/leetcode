# 1310. XOR Queries of a Subarray
Given the array ```arr``` of positive integers and the array ```queries``` where <code>queries[i] = [L<sub>i</sub>, R<sub>i</sub>]</code> for each query ```i``` compute the **XOR** of elements from <code>L<sub>i</sub></code> to <code>R<sub>i</sub></code> (that is, <code>arr[L<sub>i</sub>] <strong>xor</strong> arr[L<sub>i</sub>+1] <strong>xor</strong> ... <strong>xor</strong> arr[R<sub>i</sub>]</code> ). Return an array containing the result for the given ```queries```.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,3,4,8], queries = [[0,1],[1,2],[0,3],[3,3]]
<strong>Output:</strong> [2,7,14,8]
<strong>Explanation:</strong>
The binary representation of the elements in the array are:
1 = 0001
3 = 0011
4 = 0100
8 = 1000
The XOR values for queries are:
[0,1] = 1 xor 3 = 2
[1,2] = 3 xor 4 = 7
[0,3] = 1 xor 3 xor 4 xor 8 = 14
[3,3] = 8
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [4,8,2,10], queries = [[2,3],[1,3],[0,0],[0,3]]
<strong>Output:</strong> [8,0,4,4]
</pre>

#### Constraints:
* ```1 <= arr.length <= 3 * 10^4```
* ```1 <= arr[i] <= 10^9```
* ```1 <= queries.length <= 3 * 10^4```
* ```queries[i].length == 2```
* ```0 <= queries[i][0] <= queries[i][1] < arr.length```

## Solutions (Rust)

### 1. Prefix Sum
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
