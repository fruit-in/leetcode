# 849. Maximize Distance to Closest Person
In a row of ```seats```, ```1``` represents a person sitting in that seat, and ```0``` represents that the seat is empty.

There is at least one empty seat, and at least one person sitting.

Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized.

Return that maximum distance to closest person.

#### Example 1:
<pre>
<strong>Input:</strong> [1,0,0,0,1,0,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
If Alex sits in the second open seat (seats[2]), then the closest person has distance 2.
If Alex sits in any other open seat, the closest person has distance 1.
Thus, the maximum distance to the closest person is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,0,0,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
If Alex sits in the last seat, the closest person is 3 seats away.
This is the maximum distance possible, so the answer is 3.
</pre>

#### Note:
1. ```1 <= seats.length <= 20000```
2. ```seats``` contains only 0s or 1s, at least one ```0```, and at least one ```1```.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_distance = 0;

        for i in 0..seats.len() {
            if seats[i] == 0 {
                let mut left_distance = 0;
                for j in (0..=i).rev() {
                    if seats[j] == 1 {
                        break;
                    }
                    left_distance += 1;
                    if j == 0 {
                        left_distance += 20000;
                    }
                }

                let mut right_distance = 0;
                for j in i..seats.len() {
                    if seats[j] == 1 {
                        break;
                    }
                    right_distance += 1;
                    if j == seats.len() - 1{
                        right_distance += 20000;
                    }
                }

                max_distance = max_distance.max(left_distance.min(right_distance));
            }
        }

        max_distance
    }
}
```

### 2. Count Adjacent Zeroes
```Rust
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_empty = 0;

        let mut i = -1;
        for j in 0..(seats.len() + 1) {
            if j == seats.len() {
                max_empty = max_empty.max(2 * (j as i32 - i - 1));
            } else if seats[j] == 1 {
                if i == -1 {
                    max_empty = max_empty.max(2 * (j as i32 - i - 1));
                } else {
                    max_empty = max_empty.max(j as i32 - i - 1);
                }
                i = j as i32;
            }
        }

        (max_empty + 1) / 2
    }
}
```
