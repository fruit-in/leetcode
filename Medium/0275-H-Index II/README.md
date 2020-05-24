# 275. H-Index II
Given an array of citations **sorted in ascending order** (each citation is a non-negative integer) of a researcher, write a function to compute the researcher's h-index.

According to the [definition of h-index on Wikipedia](https://en.wikipedia.org/wiki/H-index): "A scientist has index *h* if *h* of his/her *N* papers have **at least** *h* citations each, and the other *N − h* papers have **no more than** *h* citations each."

#### Example:
<pre>
<strong>Input:</strong> citations = [0,1,3,5,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> [0,1,3,5,6] means the researcher has 5 papers in total and each of them had 
             received 0, 1, 3, 5, 6 citations respectively. 
             Since the researcher has 3 papers with <strong>at least</strong> 3 citations each and the remaining 
             two with <strong>no more than</strong> 3 citations each, her h-index is 3.
</pre>

#### Note:
If there are several possible values for *h*, the maximum one is taken as the h-index.

#### Follow up:
* This is a follow up problem to [H-Index](https://leetcode.com/problems/h-index/description/), where `citations` is now guaranteed to be sorted in ascending order.
* Could you solve it in logarithmic time complexity?

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let len = citations.len();
        let mut l = 0;
        let mut r = len;
        let mut ret = 0;

        while l < r {
            let m = (l + r) / 2;

            if citations[m] as usize <= len - m {
                ret = ret.max(citations[m]);
                l = m + 1;
            } else {
                ret = ret.max((len - m) as i32);
                r = m;
            }
        }

        ret
    }
}
```
