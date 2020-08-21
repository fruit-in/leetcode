# 1052. Grumpy Bookstore Owner
Today, the bookstore owner has a store open for ```customers.length``` minutes.  Every minute, some number of customers (```customers[i]```) enter the store, and all those customers leave after the end of that minute.

On some minutes, the bookstore owner is grumpy.  If the bookstore owner is grumpy on the i-th minute, ```grumpy[i] = 1```, otherwise ```grumpy[i] = 0```.  When the bookstore owner is grumpy, the customers of that minute are not satisfied, otherwise they are satisfied.

The bookstore owner knows a secret technique to keep themselves not grumpy for ```X``` minutes straight, but can only use it once.

Return the maximum number of customers that can be satisfied throughout the day.

#### Example 1:
<pre>
<strong>Input:</strong> customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], X = 3
<strong>Output:</strong> 16
<strong>Explanation:</strong> The bookstore owner keeps themselves not grumpy for the last 3 minutes. 
The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 + 5 = 16.
</pre>

#### Note:
* ```1 <= X <= customers.length == grumpy.length <= 20000```
* ```0 <= customers[i] <= 1000```
* ```0 <= grumpy[i] <= 1```

## Solutions (Ruby)

### 1. Sliding Window
```Ruby
# @param {Integer[]} customers
# @param {Integer[]} grumpy
# @param {Integer} x
# @return {Integer}
def max_satisfied(customers, grumpy, x)
    curr = 0
    ret = 0

    for i in 0...grumpy.length
        if i < x or grumpy[i] == 0
            curr += customers[i]
            ret += customers[i]
        end
        curr -= customers[i - x] if i >= x and grumpy[i - x] == 1
        curr += customers[i] if i >= x and grumpy[i] == 1

        ret = [ret, curr].max
    end

    return ret
end
```

## Solutions (Rust)

### 1. Sliding Window
```Rust
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut curr = 0;
        let mut ret = 0;

        for i in 0..grumpy.len() {
            if i < x as usize || grumpy[i] == 0 {
                curr += customers[i];
                ret += customers[i];
            }
            if i >= x as usize && grumpy[i - x as usize] == 1 {
                curr -= customers[i - x as usize];
            }
            if i >= x as usize && grumpy[i] == 1 {
                curr += customers[i];
            }

            ret = ret.max(curr);
        }

        ret
    }
}
```
