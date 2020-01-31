# 120. Triangle
Given a triangle, find the minimum path sum from top to bottom. Each step you may move to adjacent numbers on the row below.

For example, given the following triangle

<pre>
[
     [<strong>2</strong>],
    [<strong>3</strong>,4],
   [6,<strong>5</strong>,7],
  [4,<strong>1</strong>,8,3]
]
</pre>

The minimum path sum from top to bottom is ```11``` (i.e., **2** + **3** + **5** + **1** = 11).

#### Note:
Bonus point if you are able to do this using only *O*(*n*) extra space, where *n* is the total number of rows in the triangle.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle;

        for r in 1..triangle.len() {
            triangle[r][0] += triangle[r - 1][0];
            triangle[r][r] += triangle[r - 1][r - 1];
            for i in 1..(triangle[r].len() - 1) {
                triangle[r][i] += triangle[r - 1][i - 1].min(triangle[r - 1][i])
            }
        }

        *triangle.last().unwrap().iter().min().unwrap()
    }
}
```
