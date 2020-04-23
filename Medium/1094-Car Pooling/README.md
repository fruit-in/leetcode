# 1094. Car Pooling
You are driving a vehicle that has ```capacity``` empty seats initially available for passengers.  The vehicle **only** drives east (ie. it **cannot** turn around and drive west.)

Given a list of ```trips```, ```trip[i] = [num_passengers, start_location, end_location]``` contains information about the ```i```-th trip: the number of passengers that must be picked up, and the locations to pick them up and drop them off.  The locations are given as the number of kilometers due east from your vehicle's initial location.

Return ```true``` if and only if it is possible to pick up and drop off all passengers for all the given trips.

#### Example 1:
<pre>
<strong>Input:</strong> trips = [[2,1,5],[3,3,7]], capacity = 4
<strong>Output:</strong> false
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> trips = [[2,1,5],[3,3,7]], capacity = 5
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> trips = [[2,1,5],[3,5,7]], capacity = 3
<strong>Output:</strong> true
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> trips = [[3,2,7],[3,7,9],[8,3,9]], capacity = 11
<strong>Output:</strong> true
</pre>

#### Constraints:
1. ```trips.length <= 1000```
2. ```trips[i].length == 3```
3. ```1 <= trips[i][0] <= 100```
4. ```0 <= trips[i][1] < trips[i][2] <= 1000```
5. ```1 <= capacity <= 100000```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pick = [0; 1001];

        for trip in trips {
            pick[trip[2] as usize] -= trip[0];
            pick[trip[1] as usize] += trip[0];
        }

        for i in 1..1001 {
            pick[i] += pick[i - 1];
            if pick[i] > capacity {
                return false;
            }
        }

        pick[0] <= capacity
    }
}
```
