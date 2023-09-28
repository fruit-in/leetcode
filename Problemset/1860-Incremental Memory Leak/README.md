# 1860. Incremental Memory Leak
You are given two integers `memory1` and `memory2` representing the available memory in bits on two memory sticks. There is currently a faulty program running that consumes an increasing amount of memory every second.

At the <code>i<sup>th</sup></code> second (starting from 1), `i` bits of memory are allocated to the stick with **more available memory** (or from the first memory stick if both have the same available memory). If neither stick has at least `i` bits of available memory, the program **crashes**.

Return *an array containing* <code>[crashTime, memory1<sub>crash</sub>, memory2<sub>crash</sub>]</code>, *where* `crashTime` *is the time (in seconds) when the program crashed and* <code>memory1<sub>crash</sub></code> *and* <code>memory2<sub>crash</sub></code> *are the available bits of memory in the first and second sticks respectively*.

#### Example 1:
<pre>
<strong>Input:</strong> memory1 = 2, memory2 = 2
<strong>Output:</strong> [3,1,0]
<strong>Explanation:</strong> The memory is allocated as follows:
- At the 1st second, 1 bit of memory is allocated to stick 1. The first stick now has 1 bit of available memory.
- At the 2nd second, 2 bits of memory are allocated to stick 2. The second stick now has 0 bits of available memory.
- At the 3rd second, the program crashes. The sticks have 1 and 0 bits available respectively.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> memory1 = 8, memory2 = 11
<strong>Output:</strong> [6,0,4]
<strong>Explanation:</strong> The memory is allocated as follows:
- At the 1st second, 1 bit of memory is allocated to stick 2. The second stick now has 10 bit of available memory.
- At the 2nd second, 2 bits of memory are allocated to stick 2. The second stick now has 8 bits of available memory.
- At the 3rd second, 3 bits of memory are allocated to stick 1. The first stick now has 5 bits of available memory.
- At the 4th second, 4 bits of memory are allocated to stick 2. The second stick now has 4 bits of available memory.
- At the 5th second, 5 bits of memory are allocated to stick 1. The first stick now has 0 bits of available memory.
- At the 6th second, the program crashes. The sticks have 0 and 4 bits available respectively.
</pre>

#### Constraints:
* <code>0 <= memory1, memory2 <= 2<sup>31</sup> - 1</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut bits = 1;
        let mut crash_time = 1;
        let mut memory1 = memory1;
        let mut memory2 = memory2;

        while memory1.max(memory2) >= bits {
            crash_time += 1;
            if memory1 >= memory2 {
                memory1 -= bits;
            } else {
                memory2 -= bits;
            }
            bits += 1;
        }

        vec![crash_time, memory1, memory2]
    }
}
```