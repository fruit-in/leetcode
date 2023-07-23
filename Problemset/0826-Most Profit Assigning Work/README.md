# 826. Most Profit Assigning Work
We have jobs: `difficulty[i]` is the difficulty of the `i`th job, and `profit[i]` is the profit of the `i`th job.

Now we have some workers. `worker[i]` is the ability of the `i`th worker, which means that this worker can only complete a job with difficulty at most `worker[i]`.

Every worker can be assigned at most one job, but one job can be completed multiple times.

For example, if 3 people attempt the same job that pays $1, then the total profit will be $3.  If a worker cannot complete any job, his profit is $0.

What is the most profit we can make?

#### Example 1:
<pre>
<strong>Input:</strong> difficulty = [2,4,6,8,10], profit = [10,20,30,40,50], worker = [4,5,6,7]
<strong>Output:</strong> 100
<strong>Explanation:</strong> Workers are assigned jobs of difficulty [4,4,6,6] and they get profit of [20,20,30,30] seperately.
</pre>

#### Notes:
* `1 <= difficulty.length = profit.length <= 10000`
* `1 <= worker.length <= 10000`
* `difficulty[i], profit[i], worker[i]`  are in range `[1, 10^5]`

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut work = difficulty
            .into_iter()
            .zip(profit.into_iter())
            .collect::<Vec<_>>();
        work.sort_unstable();
        let mut ret = 0;

        for i in 1..work.len() {
            work[i].1 = work[i].1.max(work[i - 1].1);
        }

        for w in worker {
            ret += match work.binary_search_by_key(&w, |&(d, _)| d) {
                Ok(i) => work[i].1,
                Err(0) => 0,
                Err(i) => work[i - 1].1,
            };
        }

        ret
    }
}
```
