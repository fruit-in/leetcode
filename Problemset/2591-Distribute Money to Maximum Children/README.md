# 2591. Distribute Money to Maximum Children
You are given an integer `money` denoting the amount of money (in dollars) that you have and another integer `children` denoting the number of children that you must distribute the money to.

You have to distribute the money according to the following rules:

* All money must be distributed.
* Everyone must receive at least `1` dollar.
* Nobody receives `4` dollars.

Return *the **maximum** number of children who may receive **exactly*** `8` *dollars if you distribute the money according to the aforementioned rules*. If there is no way to distribute the money, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> money = 20, children = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The maximum number of children with 8 dollars will be 1. One of the ways to distribute the money is:
- 8 dollars to the first child.
- 9 dollars to the second child.
- 3 dollars to the third child.
It can be proven that no distribution exists such that number of children getting 8 dollars is greater than 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> money = 16, children = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> Each child can be given 8 dollars.
</pre>

#### Constraints:
* `1 <= money <= 200`
* `2 <= children <= 30`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        let money = money - children;

        if money < 0 {
            -1
        } else if money / 7 == children && money % 7 == 0 {
            children
        } else if money / 7 >= children {
            children - 1
        } else if money / 7 == children - 1 && money % 7 == 3 {
            children - 2
        } else {
            money / 7
        }
    }
}
```
