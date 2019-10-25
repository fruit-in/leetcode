# 1010. Pairs of Songs With Total Durations Divisible by 60
In a list of songs, the ```i```-th song has a duration of ```time[i]``` seconds.

Return the number of pairs of songs for which their total duration in seconds is divisible by ```60```.  Formally, we want the number of indices ```i < j``` with ```(time[i] + time[j]) % 60 == 0```.

#### Example 1:
<pre>
<strong>Input:</strong> [30,20,150,100,40]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Three pairs have a total duration divisible by 60:
(time[0] = 30, time[2] = 150): total duration 180
(time[1] = 20, time[3] = 100): total duration 120
(time[1] = 20, time[4] = 40): total duration 60
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [60,60,60]
<strong>Output:</strong> 3
<strong>Explanation:</strong> All three pairs have a total duration of 120, which is divisible by 60.
</pre>

#### Note:
1. ```1 <= time.length <= 60000```
2. ```1 <= time[i] <= 500```

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 0..time.len() {
            for j in (i + 1)..time.len() {
                if (time[i] + time[j]) % 60 == 0 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```

### 2. Count
```Rust
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = [0; 60];
        let mut ret = 0;

        for t in time {
            ret += match t % 60 {
                0 => cnt[0],
                _ => cnt[60 - t as usize % 60],
            };
            cnt[t as usize % 60] += 1;
        }

        ret
    }
}
```
