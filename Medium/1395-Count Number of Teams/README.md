# 1395. Count Number of Teams
There are ```n``` soldiers standing in a line. Each soldier is assigned a **unique** ```rating``` value.

You have to form a team of 3 soldiers amongst them under the following rules:
* Choose 3 soldiers with index (```i```, ```j```, ```k```) with rating (```rating[i]```, ```rating[j]```, ```rating[k]```).
* A team is valid if:  (```rating[i] < rating[j] < rating[k]```) or (```rating[i] > rating[j] > rating[k]```) where (```0 <= i < j < k < n```).

Return the number of teams you can form given the conditions. (soldiers can be part of multiple teams).

#### Example 1:
<pre>
<strong>Input:</strong> rating = [2,5,3,4,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can form three teams given the conditions. (2,3,4), (5,4,1), (5,3,1).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rating = [2,1,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> We can't form any team given the conditions.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> rating = [1,2,3,4]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* ```n == rating.length```
* ```1 <= n <= 200```
* ```1 <= rating[i] <= 10^5```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut ret = 0;

        for j in 1..(rating.len() - 1) {
            let ltl = rating[..j].iter().filter(|&&x| x < rating[j]).count();
            let gtl = j - ltl;
            let ltr = rating[j..].iter().filter(|&&x| x < rating[j]).count();
            let gtr = rating.len() - 1 - j - ltr;

            ret += ltl * gtr + ltr * gtl;
        }

        ret as i32
    }
}
```
