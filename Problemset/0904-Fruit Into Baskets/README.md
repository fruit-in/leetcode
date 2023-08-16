# 904. Fruit Into Baskets
You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array `fruits` where `fruits[i]` is the **type** of fruit the <code>i<sup>th</sup></code> tree produces.

You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:

* You only have **two** baskets, and each basket can only hold a **single type** of fruit. There is no limit on the amount of fruit each basket can hold.
* Starting from any tree of your choice, you must pick **exactly one fruit** from **every** tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
* Once you reach a tree with fruit that cannot fit in your baskets, you must stop.

Given the integer array `fruits`, return *the **maximum** number of fruits you can pick*.

#### Example 1:
<pre>
<strong>Input:</strong> fruits = [1,2,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can pick from all 3 trees.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> fruits = [0,1,2,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can pick from trees [1,2,2].
If we had started at the first tree, we would only pick from trees [0,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> fruits = [1,2,3,2,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can pick from trees [2,3,2,2].
If we had started at the first tree, we would only pick from trees [1,2].
</pre>

#### Constraints:
* <code>1 <= fruits.length <= 10<sup>5</sup></code>
* `0 <= fruits[i] < fruits.length`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut count = HashMap::from([(fruits[0], 1)]);
        let mut r = 0;
        let mut ret = 0;

        for l in 0..fruits.len() {
            while count.len() <= 2 {
                ret = ret.max(r - l + 1);

                if r + 1 == fruits.len() {
                    break;
                }
                r += 1;
                count.entry(fruits[r]).and_modify(|c| *c += 1).or_insert(1);
            }

            if count[&fruits[l]] == 1 {
                count.remove(&fruits[l]);
            } else {
                *count.get_mut(&fruits[l]).unwrap() -= 1;
            }
        }

        ret as i32
    }
}
```
