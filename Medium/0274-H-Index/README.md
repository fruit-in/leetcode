# 274. H-Index
Given an array of citations (each citation is a non-negative integer) of a researcher, write a function to compute the researcher's h-index.

According to the [definition of h-index on Wikipedia](https://en.wikipedia.org/wiki/H-index): "A scientist has index *h* if *h* of his/her *N* papers have **at least** *h* citations each, and the other *N − h* papers have **no more than** *h* citations each."

#### Example:
<pre>
<strong>Input:</strong> citations = [3,0,6,1,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> [3,0,6,1,5] means the researcher has 5 papers in total and each of them had 
             received 3, 0, 6, 1, 5 citations respectively. 
             Since the researcher has 3 papers with <strong>at least</strong> 3 citations each and the remaining 
             two with <strong>no more than</strong> 3 citations each, her h-index is 3.
</pre>

#### Note:
If there are several possible values for *h*, the maximum one is taken as the h-index.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_unstable_by(|a, b| b.cmp(a));
        let mut h = 0;

        for i in 0..citations.len() {
            h = h.max(citations[i].min(i as i32 + 1));
        }

        h
    }
}
```

### 2. Count
```Rust
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut counter = vec![0; citations.len() + 1];
        for &c in &citations {
            counter[(c as usize).min(citations.len())] += 1;
        }

        let mut s = 0;
        let mut i = counter.len() - 1;
        loop {
            s += counter[i];
            if s >= i {
                return i as i32;
            }
            i -= 1;
        }
    }
}
```
