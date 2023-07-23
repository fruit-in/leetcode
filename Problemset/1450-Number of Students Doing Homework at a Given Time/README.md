# 1450. Number of Students Doing Homework at a Given Time
Given two integer arrays `startTime` and `endTime` and given an integer `queryTime`.

The `ith` student started doing their homework at the time `startTime[i]` and finished it at time `endTime[i]`.

Return *the number of students* doing their homework at time `queryTime`. More formally, return the number of students where `queryTime` lays in the interval `[startTime[i], endTime[i]]` inclusive.

#### Example 1:
<pre>
<strong>Input:</strong> startTime = [1,2,3], endTime = [3,2,7], queryTime = 4
<strong>Output:</strong> 1
<strong>Explanation:</strong> We have 3 students where:
The first student started doing homework at time 1 and finished at time 3 and wasn't doing anything at time 4.
The second student started doing homework at time 2 and finished at time 2 and also wasn't doing anything at time 4.
The third student started doing homework at time 3 and finished at time 7 and was the only student doing homework at time 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> startTime = [4], endTime = [4], queryTime = 4
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only student was doing their homework at the queryTime.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> startTime = [4], endTime = [4], queryTime = 5
<strong>Output:</strong> 0
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> startTime = [1,1,1,1], endTime = [1,3,2,4], queryTime = 7
<strong>Output:</strong> 0
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> startTime = [9,8,7,6,5,4,3,2,1], endTime = [10,10,10,10,10,10,10,10,10], queryTime = 5
<strong>Output:</strong> 5
</pre>

#### Constraints:
* `startTime.length == endTime.length`
* `1 <= startTime.length <= 100`
* `1 <= startTime[i] <= endTime[i] <= 1000`
* `1 <= queryTime <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .filter(|(&s, &e)| s <= query_time && query_time <= e)
            .count() as i32
    }
}
```
