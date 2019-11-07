# 455. Assign Cookies
Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie. Each child i has a greed factor g<sub>i</sub>, which is the minimum size of a cookie that the child will be content with; and each cookie j has a size s<sub>j</sub>. If s<sub>j</sub> >= g<sub>i</sub>, we can assign the cookie j to the child i, and the child i will be content. Your goal is to maximize the number of your content children and output the maximum number.

#### Note:
You may assume the greed factor is always positive.<br>
You cannot assign more than one cookie to one child.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3], [1,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You have 3 children and 2 cookies. The greed factors of 3 children are 1, 2, 3. 
And even though you have 2 cookies, since their size is both 1, you could only make the child whose greed factor is 1 content.
You need to output 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2], [1,2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> You have 2 children and 3 cookies. The greed factors of 2 children are 1, 2. 
You have 3 cookies and their sizes are big enough to gratify all of the children, 
You need to output 2.
</pre>

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable();
        s.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut ret = 0;

        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                ret += 1;
                i += 1;
            }
            j += 1;
        }

        ret
    }
}
```
