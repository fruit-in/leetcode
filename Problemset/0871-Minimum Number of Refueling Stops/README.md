# 871. Minimum Number of Refueling Stops
A car travels from a starting position to a destination which is `target` miles east of the starting position.

There are gas stations along the way. The gas stations are represented as an array `stations` where <code>stations[i] = [position<sub>i</sub>, fuel<sub>i</sub>]</code> indicates that the <code>i<sup>th</sup></code> gas station is <code>position<sub>i</sub></code> miles east of the starting position and has <code>fuel<sub>i</sub></code> liters of gas.

The car starts with an infinite tank of gas, which initially has `startFuel` liters of fuel in it. It uses one liter of gas per one mile that it drives. When the car reaches a gas station, it may stop and refuel, transferring all the gas from the station into the car.

Return *the minimum number of refueling stops the car must make in order to reach its destination*. If it cannot reach the destination, return `-1`.

Note that if the car reaches a gas station with `0` fuel left, the car can still refuel there. If the car reaches the destination with `0` fuel left, it is still considered to have arrived.

#### Example 1:
<pre>
<strong>Input:</strong> target = 1, startFuel = 1, stations = []
<strong>Output:</strong> 0
<strong>Explanation:</strong> We can reach the target without refueling.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = 100, startFuel = 1, stations = [[10,100]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> We can not reach the target (or even the first gas station).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = 100, startFuel = 10, stations = [[10,60],[20,30],[30,30],[60,40]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We start with 10 liters of fuel.
We drive to position 10, expending 10 liters of fuel.  We refuel from 0 liters to 60 liters of gas.
Then, we drive from position 10 to position 60 (expending 50 liters of fuel),
and refuel from 10 liters to 50 liters of gas.  We then drive to and reach the target.
We made 2 refueling stops along the way, so we return 2.
</pre>

#### Constraints:
* <code>1 <= target, startFuel <= 10<sup>9</sup></code>
* `0 <= stations.length <= 500`
* <code>1 <= position<sub>i</sub> < position<sub>i+1</sub> < target</code>
* <code>1 <= fuel<sub>i</sub> < 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut fuel = start_fuel;
        let mut position = 0;
        let mut fuels = BinaryHeap::new();

        for station in &stations {
            while position + fuel < station[0] {
                match fuels.pop() {
                    Some(f) => fuel += f,
                    None => return -1,
                }
            }

            fuel -= station[0] - position;
            position = station[0];
            fuels.push(station[1]);
        }

        while position + fuel < target {
            match fuels.pop() {
                Some(f) => fuel += f,
                None => return -1,
            }
        }

        (stations.len() - fuels.len()) as i32
    }
}
```
