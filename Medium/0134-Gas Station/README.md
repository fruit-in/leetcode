# 134. Gas Station
There are *N* gas stations along a circular route, where the amount of gas at station *i* is ```gas[i]```.

You have a car with an unlimited gas tank and it costs ```cost[i]``` of gas to travel from station *i* to its next station (*i*+1). You begin the journey with an empty tank at one of the gas stations.

Return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1.

#### Note:
* If there exists a solution, it is guaranteed to be unique.
* Both input arrays are non-empty and have the same length.
* Each element in the input arrays is a non-negative integer.

#### Example 1:
<pre>
<strong>Input:</strong>
gas  = [1,2,3,4,5]
cost = [3,4,5,1,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Start at station 3 (index 3) and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
Travel to station 4. Your tank = 4 - 1 + 5 = 8
Travel to station 0. Your tank = 8 - 2 + 1 = 7
Travel to station 1. Your tank = 7 - 3 + 2 = 6
Travel to station 2. Your tank = 6 - 4 + 3 = 5
Travel to station 3. The cost is 5. Your gas is just enough to travel back to station 3.
Therefore, return 3 as the starting index.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
gas  = [2,3,4]
cost = [3,4,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
You can't start at station 0 or 1, as there is not enough gas to travel to the next station.
Let's start at station 2 and fill up with 4 unit of gas. Your tank = 0 + 4 = 4
Travel to station 0. Your tank = 4 - 3 + 2 = 3
Travel to station 1. Your tank = 3 - 3 + 3 = 3
You cannot travel back to station 2, as it requires 4 unit of gas but you only have 3.
Therefore, you can't travel around the circuit once no matter where you start.
</pre>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} gas
# @param {Integer[]} cost
# @return {Integer}
def can_complete_circuit(gas, cost)
    start = 0
    tank = 0

    for i in 0...(2 * gas.length)
        j = i % gas.length

        tank += gas[j] - cost[j]

        if tank >= 0 and (j + 1) % gas.length == start
            return start
        elsif tank < 0
            start = j + 1
            tank = 0
        end
    end

    return -1
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut tank = 0;

        for i in 0..(2 * gas.len()) {
            let j = i % gas.len();

            tank += gas[j] - cost[j];

            if tank >= 0 && (j + 1) % gas.len() == start {
                return start as i32;
            } else if tank < 0 {
                start = j + 1;
                tank = 0;
            }
        }

        -1
    }
}
```
