# 1298. Maximum Candies You Can Get from Boxes
You have `n` boxes labeled from `0` to `n - 1`. You are given four arrays: `status`, `candies`, `keys`, and `containedBoxes` where:

* `status[i]` is `1` if the <code>i<sup>th</sup></code> box is open and `0` if the <code>i<sup>th</sup></code> box is closed,
* `candies[i]` is the number of candies in the <code>i<sup>th</sup></code> box,
* `keys[i]` is a list of the labels of the boxes you can open after opening the <code>i<sup>th</sup></code> box.
* `containedBoxes[i]` is a list of the boxes you found inside the <code>i<sup>th</sup></code> box.

You are given an integer array `initialBoxes` that contains the labels of the boxes you initially have. You can take all the candies in **any open box** and you can use the keys in it to open new boxes and you also can use the boxes you find in it.

Return *the maximum number of candies you can get following the rules above*.

#### Example 1:
<pre>
<strong>Input:</strong> status = [1,0,1,0], candies = [7,5,4,100], keys = [[],[],[1],[]], containedBoxes = [[1,2],[3],[],[]], initialBoxes = [0]
<strong>Output:</strong> 16
<strong>Explanation:</strong> You will be initially given box 0. You will find 7 candies in it and boxes 1 and 2.
Box 1 is closed and you do not have a key for it so you will open box 2. You will find 4 candies and a key to box 1 in box 2.
In box 1, you will find 5 candies and box 3 but you will not find a key to box 3 so box 3 will remain closed.
Total number of candies collected = 7 + 4 + 5 = 16 candy.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> status = [1,0,0,0,0,0], candies = [1,1,1,1,1,1], keys = [[1,2,3,4,5],[],[],[],[],[]], containedBoxes = [[1,2,3,4,5],[],[],[],[],[]], initialBoxes = [0]
<strong>Output:</strong> 6
<strong>Explanation:</strong> You have initially box 0. Opening it you can find boxes 1,2,3,4 and 5 and their keys.
The total number of candies will be 6.
</pre>

#### Constraints:
* `n == status.length == candies.length == keys.length == containedBoxes.length`
* `1 <= n <= 1000`
* `status[i]` is either `0` or `1`.
* `1 <= candies[i] <= 1000`
* `0 <= keys[i].length <= n`
* `0 <= keys[i][j] < n`
* All values of `keys[i]` are **unique**.
* `0 <= containedBoxes[i].length <= n`
* `0 <= containedBoxes[i][j] < n`
* All values of `containedBoxes[i]` are unique.
* Each box is contained in one box at most.
* `0 <= initialBoxes.length <= n`
* `0 <= initialBoxes[i] < n`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut closed = HashSet::new();
        let mut opened = vec![];
        let mut used = HashSet::new();
        let mut ret = 0;

        for b in initial_boxes {
            if status[b as usize] == 1 && !used.contains(&b) {
                opened.push(b);
                used.insert(b);
            } else if status[b as usize] == 0 {
                closed.insert(b);
            }
        }

        while let Some(b) = opened.pop() {
            ret += candies[b as usize];

            for &k in &keys[b as usize] {
                status[k as usize] = 1;
                if closed.remove(&k) && !used.contains(&k) {
                    opened.push(k);
                    used.insert(k);
                }
            }

            for &cb in &contained_boxes[b as usize] {
                if status[cb as usize] == 1 && !used.contains(&cb) {
                    opened.push(cb);
                    used.insert(cb);
                } else if status[cb as usize] == 0 {
                    closed.insert(cb);
                }
            }
        }

        ret
    }
}
```
