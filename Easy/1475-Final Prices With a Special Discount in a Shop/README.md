# 1475. Final Prices With a Special Discount in a Shop
Given the array `prices` where `prices[i]` is the price of the `ith` item in a shop. There is a special discount for items in the shop, if you buy the `ith` item, then you will receive a discount equivalent to `prices[j]` where `j` is the **minimum** index such that `j > i` and `prices[j] <= prices[i]`, otherwise, you will not receive any discount at all.

*Return an array where the `ith` element is the final price you will pay for the `ith` item of the shop considering the special discount.*

#### Example 1:
<pre>
<strong>Input:</strong> prices = [8,4,6,2,3]
<strong>Output:</strong> [4,2,4,2,3]
<strong>Explanation:</strong>
For item 0 with price[0]=8 you will receive a discount equivalent to prices[1]=4, therefore, the final price you will pay is 8 - 4 = 4.
For item 1 with price[1]=4 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 4 - 2 = 2.
For item 2 with price[2]=6 you will receive a discount equivalent to prices[3]=2, therefore, the final price you will pay is 6 - 2 = 4.
For items 3 and 4 you will not receive any discount at all.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> prices = [1,2,3,4,5]
<strong>Output:</strong> [1,2,3,4,5]
<strong>Explanation:</strong> In this case, for all items, you will not receive any discount at all.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> prices = [10,1,1,6]
<strong>Output:</strong> [9,0,1,6]
</pre>

#### Constraints:
* `1 <= prices.length <= 500`
* `1 <= prices[i] <= 10^3`

## Solutions (Rust)

### 1. Stack
```Rust
impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![0; prices.len()];

        for i in (0..prices.len()).rev() {
            while *stack.last().unwrap_or(&0) > prices[i] {
                stack.pop();
            }
            ret[i] = prices[i] - *stack.last().unwrap_or(&0);
            stack.push(prices[i]);
        }

        ret
    }
}
```
