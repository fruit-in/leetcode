# 1725. Number Of Rectangles That Can Form The Largest Square
You are given an array `rectangles` where <code>rectangles[i] = [l<sub>i</sub>, w<sub>i</sub>]</code> represents the <code>i<sup>th</sup></code> rectangle of length <code>l<sub>i</sub></code> and width <code>w<sub>i</sub></code>.

You can cut the <code>i<sup>th</sup></code> rectangle to form a square with a side length of `k` if both <code>k <= l<sub>i</sub></code> and <code>k <= w<sub>i</sub></code>. For example, if you have a rectangle `[4,6]`, you can cut it to get a square with a side length of at most `4`.

Let `maxLen` be the side length of the **largest** square you can obtain from any of the given rectangles.

Return *the **number** of rectangles that can make a square with a side length of* `maxLen`.

#### Example 1:
<pre>
<strong>Input:</strong> rectangles = [[5,8],[3,9],[5,12],[16,5]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The largest squares you can get from each rectangle are of lengths [5,3,5,5].
The largest possible square is of length 5, and you can get it out of 3 rectangles.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rectangles = [[2,3],[3,7],[4,3],[3,7]]
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `1 <= rectangles.length <= 1000`
* `rectangles[i].length == 2`
* <code>1 <= l<sub>i</sub>, w<sub>i</sub> <= 10<sup>9</sup></code>
* <code>l<sub>i</sub> != w<sub>i</sub></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} rectangles
# @return {Integer}
def count_good_rectangles(rectangles)
  max_len = 0
  ret = 0

  rectangles.each do |rec|
    if rec.min == max_len
      ret += 1
    elsif rec.min > max_len
      max_len = rec.min
      ret = 1
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_len = 0;
        let mut ret = 0;

        for rec in rectangles {
            if rec[0].min(rec[1]) == max_len {
                ret += 1;
            } else if rec[0].min(rec[1]) > max_len {
                max_len = rec[0].min(rec[1]);
                ret = 1;
            }
        }

        ret
    }
}
```
