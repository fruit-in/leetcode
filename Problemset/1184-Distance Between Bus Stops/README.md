# 1184. Distance Between Bus Stops
A bus has ```n``` stops numbered from ```0``` to ```n - 1``` that form a circle. We know the distance between all pairs of neighboring stops where ```distance[i]``` is the distance between the stops number ```i``` and ```(i + 1) % n```.

The bus goes along both directions i.e. clockwise and counterclockwise.

Return the shortest distance between the given ```start``` and ```destination``` stops.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/03/untitled-diagram-1.jpg)
<pre>
<strong>Input:</strong> distance = [1,2,3,4], start = 0, destination = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> Distance between 0 and 1 is 1 or 9, minimum is 1.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/09/03/untitled-diagram-1-1.jpg)
<pre>
<strong>Input:</strong> distance = [1,2,3,4], start = 0, destination = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> Distance between 0 and 2 is 3 or 7, minimum is 3.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/09/03/untitled-diagram-1-2.jpg)
<pre>
<strong>Input:</strong> distance = [1,2,3,4], start = 0, destination = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> Distance between 0 and 3 is 6 or 4, minimum is 4.
</pre>

#### Constraints:
* ```1 <= n <= 10^4```
* ```distance.length == n```
* ```0 <= start, destination < n```
* ```0 <= distance[i] <= 10^4```

## Solutions (Rust)

### 1. Compare Clockwise with Counterclockwise
```Rust
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let mut counterclockwise = 0;
        curr = start;
        while curr != destination {
            curr -= 1;
            curr = (curr + n) % n;
            counterclockwise += distance[curr as usize];
        }
        clockwise.min(counterclockwise)
    }
}
```

### 2. Total Distance Subtract Clockwise
```Rust
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let total_distance: i32 = distance.iter().sum();
        clockwise.min(total_distance - clockwise)
    }
}
```

### 3. Swap Start for Destination
```Rust
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let mut counterclockwise = 0;
        curr = destination;
        while curr != start {
            counterclockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }
        clockwise.min(counterclockwise)
    }
}
```
