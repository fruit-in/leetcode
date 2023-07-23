# 495. Teemo Attacking
In LOL world, there is a hero called Teemo and his attacking can make his enemy Ashe be in poisoned condition. Now, given the Teemo's attacking **ascending** time series towards Ashe and the poisoning time duration per Teemo's attacking, you need to output the total time that Ashe is in poisoned condition.

You may assume that Teemo attacks at the very beginning of a specific time point, and makes Ashe be in poisoned condition immediately.

#### Example 1:
<pre>
<strong>Input:</strong> [1,4], 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> At time point 1, Teemo starts attacking Ashe and makes Ashe be poisoned immediately. 
This poisoned status will last 2 seconds until the end of time point 2. 
And at time point 4, Teemo attacks Ashe again, and causes Ashe to be in poisoned status for another 2 seconds. 
So you finally need to output 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2], 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> At time point 1, Teemo starts attacking Ashe and makes Ashe be poisoned. 
This poisoned status will last 2 seconds until the end of time point 2. 
However, at the beginning of time point 2, Teemo attacks Ashe again who is already in poisoned status. 
Since the poisoned status won't add up together, though the second poisoning attack will still work at time point 2, it will stop at the end of time point 3. 
So you finally need to output 3.
</pre>

#### Note:
1. You may assume the length of given time series array won't exceed 10000.
2. You may assume the numbers in the Teemo's attacking time series and his poisoning time duration per attacking are non-negative integers, which won't exceed 10,000,000.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut end = -1;
        let mut ret = 0;

        for i in 0..time_series.len() {
            if time_series[i] >= end {
                ret += duration;
            } else {
                ret += time_series[i] - time_series[i - 1];
            }
            end = time_series[i] + duration;
        }

        ret
    }
}
```
