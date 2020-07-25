# 853. Car Fleet
`N` cars are going to the same destination along a one lane road. The destination is `target` miles away.

Each car `i` has a constant speed `speed[i]` (in miles per hour), and initial position `position[i]` miles towards the target along the road.

A car can never pass another car ahead of it, but it can catch up to it, and drive bumper to bumper at the same speed.

The distance between these two cars is ignored - they are assumed to have the same position.

A *car fleet* is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.

If a car catches up to a car fleet right at the destination point, it will still be considered as one car fleet.

How many car fleets will arrive at the destination?

#### Example 1:
<pre>
<strong>Input:</strong> target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The cars starting at 10 and 8 become a fleet, meeting each other at 12.
The car starting at 0 doesn't catch up to any other car, so it is a fleet by itself.
The cars starting at 5 and 3 become a fleet, meeting each other at 6.
Note that no other cars meet these fleets before the destination, so the answer is 3.
</pre>

#### Note:
1. `0 <= N <= 10 ^ 4`
2. `0 < target <= 10 ^ 6`
3. `0 < speed[i] <= 10 ^ 6`
4. `0 <= position[i] < target`
5. All initial positions are different.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 0 {
            return 0;
        }

        let mut cars = position.iter().zip(speed.iter()).collect::<Vec<_>>();
        cars.sort_unstable();
        let (&p, &s) = cars.pop().unwrap();
        let mut time = (target - p) as f64 / s as f64;
        let mut ret = 1;

        while let Some((&p, &s)) = cars.pop() {
            let t = (target - p) as f64 / s as f64;
            if time < t {
                ret += 1;
                time = t;
            }
        }

        ret
    }
}
```
