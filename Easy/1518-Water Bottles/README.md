# 1518. Water Bottles
Given `numBottles` full water bottles, you can exchange `numExchange` empty water bottles for one full water bottle.

The operation of drinking a full water bottle turns it into an empty bottle.

Return the **maximum** number of water bottles you can drink.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/01/sample_1_1875.png)
<pre>
<strong>Input:</strong> numBottles = 9, numExchange = 3
<strong>Output:</strong> 13
<strong>Explanation:</strong> You can exchange 3 empty bottles to get 1 full water bottle.
Number of water bottles you can drink: 9 + 3 + 1 = 13.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/07/01/sample_2_1875.png)
<pre>
<strong>Input:</strong> numBottles = 15, numExchange = 4
<strong>Output:</strong> 19
<strong>Explanation:</strong> You can exchange 4 empty bottles to get 1 full water bottle. 
Number of water bottles you can drink: 15 + 3 + 1 = 19.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> numBottles = 5, numExchange = 5
<strong>Output:</strong> 6
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> numBottles = 2, numExchange = 3
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `1 <= numBottles <= 100`
* `2 <= numExchange <= 100`

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer} num_bottles
# @param {Integer} num_exchange
# @return {Integer}
def num_water_bottles(num_bottles, num_exchange)
    ret = num_empty = num_bottles

    while num_empty >= num_exchange
        ret += num_empty / num_exchange
        num_empty = num_empty % num_exchange + num_empty / num_exchange
    end

    return ret
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut num_empty = num_bottles;
        let mut ret = num_bottles;

        while num_empty >= num_exchange {
            ret += num_empty / num_exchange;
            num_empty = num_empty % num_exchange + num_empty / num_exchange;
        }

        ret
    }
}
```
