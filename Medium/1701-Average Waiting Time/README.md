# 1701. Average Waiting Time
There is a restaurant with a single chef. You are given an array `customers`, where <code>customers[i] = [arrival<sub>i</sub>, time<sub>i</sub>]</code>:
* <code>arrival<sub>i</sub></code> is the arrival time of the <code>i<sup>th</sup></code> customer. The arrival times are sorted in **non-decreasing** order.
* <code>time<sub>i</sub></code> is the time needed to prepare the order of the <code>i<sup>th</sup></code> customer.

When a customer arrives, he gives the chef his order, and the chef starts preparing it once he is idle. The customer waits till the chef finishes preparing his order. The chef does not prepare food for more than one customer at a time. The chef prepares food for customers **in the order they were given in the input**.

Return *the **average** waiting time of all customers*. Solutions within <code>10<sup>-5</sup></code> from the actual answer are considered accepted.

#### Example 1:
<pre>
<strong>Input:</strong> customers = [[1,2],[2,5],[4,3]]
<strong>Output:</strong> 5.00000
<strong>Explanation:</strong>
1) The first customer arrives at time 1, the chef takes his order and starts preparing it immediately at time 1, and finishes at time 3, so the waiting time of the first customer is 3 - 1 = 2.
2) The second customer arrives at time 2, the chef takes his order and starts preparing it at time 3, and finishes at time 8, so the waiting time of the second customer is 8 - 2 = 6.
3) The third customer arrives at time 4, the chef takes his order and starts preparing it at time 8, and finishes at time 11, so the waiting time of the third customer is 11 - 4 = 7.
So the average waiting time = (2 + 6 + 7) / 3 = 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> customers = [[5,2],[5,4],[10,3],[20,1]]
<strong>Output:</strong> 3.25000
<strong>Explanation:</strong>
1) The first customer arrives at time 5, the chef takes his order and starts preparing it immediately at time 5, and finishes at time 7, so the waiting time of the first customer is 7 - 5 = 2.
2) The second customer arrives at time 5, the chef takes his order and starts preparing it at time 7, and finishes at time 11, so the waiting time of the second customer is 11 - 5 = 6.
3) The third customer arrives at time 10, the chef takes his order and starts preparing it at time 11, and finishes at time 14, so the waiting time of the third customer is 14 - 10 = 4.
4) The fourth customer arrives at time 20, the chef takes his order and starts preparing it immediately at time 20, and finishes at time 21, so the waiting time of the fourth customer is 21 - 20 = 1.
So the average waiting time = (2 + 6 + 4 + 1) / 4 = 3.25.
</pre>

#### Constraints:
* <code>1 <= customers.length <= 10<sup>5</sup></code>
* <code>1 <= arrival<sub>i</sub>, time<sub>i</sub> <= 10<sup>4</sup></code>
* <code>arrival<sub>i</sub> <= arrival<sub>i+1</sub></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} customers
# @return {Float}
def average_waiting_time(customers)
  time = 0
  sum = 0

  customers.each do |customer|
    if customer[0] >= time
      sum += customer[1]
      time = customer.sum
    else
      sum += time + customer[1] - customer[0]
      time += customer[1]
    end
  end

  1.0 * sum / customers.size
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut time = 0u64;
        let mut sum = 0u64;

        for customer in &customers {
            if customer[0] as u64 >= time {
                sum += customer[1] as u64;
                time = (customer[0] + customer[1]) as u64;
            } else {
                sum += time + (customer[1] - customer[0]) as u64;
                time += customer[1] as u64;
            }
        }

        sum as f64 / customers.len() as f64
    }
}
```
