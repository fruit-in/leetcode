# 2171. Removing Minimum Number of Magic Beans
You are given an array of **positive** integers `beans`, where each integer represents the number of magic beans found in a particular magic bag.

**Remove** any number of beans (**possibly none**) from each bag such that the number of beans in each remaining **non-empty** bag (still containing **at least one** bean) is **equal**. Once a bean has been removed from a bag, you are **not** allowed to return it to any of the bags.

Return *the **minimum** number of magic beans that you have to remove*.

#### Example 1:
<pre>
<strong>Input:</strong> beans = [4,1,6,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
- We remove 1 bean from the bag with only 1 bean.
  This results in the remaining bags: [4,0,6,5]
- Then we remove 2 beans from the bag with 6 beans.
  This results in the remaining bags: [4,0,4,5]
- Then we remove 1 bean from the bag with 5 beans.
  This results in the remaining bags: [4,0,4,4]
We removed a total of 1 + 2 + 1 = 4 beans to make the remaining non-empty bags have an equal number of beans.
There are no other solutions that remove 4 beans or fewer.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> beans = [2,10,3,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong>
- We remove 2 beans from one of the bags with 2 beans.
  This results in the remaining bags: [0,10,3,2]
- Then we remove 2 beans from the other bag with 2 beans.
  This results in the remaining bags: [0,10,3,0]
- Then we remove 3 beans from the bag with 3 beans.
  This results in the remaining bags: [0,10,0,0]
We removed a total of 2 + 2 + 3 = 7 beans to make the remaining non-empty bags have an equal number of beans.
There are no other solutions that removes 7 beans or fewer.
</pre>

#### Constraints:
* <code>1 <= beans.length <= 10<sup>5</sup></code>
* <code>1 <= beans[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut lsum = 0;
        let mut rsum = beans.iter().sum::<i64>();
        let mut ret = i64::MAX;
        beans.sort_unstable();

        for i in 0..beans.len() {
            ret = ret.min(lsum + rsum - (beans.len() - i) as i64 * beans[i]);
            lsum += beans[i];
            rsum -= beans[i];
        }

        ret
    }
}
```
