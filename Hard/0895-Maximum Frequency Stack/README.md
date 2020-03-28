# 895. Maximum Frequency Stack
Implement ```FreqStack```, a class which simulates the operation of a stack-like data structure.

```FreqStack``` has two functions:
* ```push(int x)```, which pushes an integer ```x``` onto the stack.
* ```pop()```, which **removes** and returns the most frequent element in the stack.
    * If there is a tie for most frequent element, the element closest to the top of the stack is removed and returned.


#### Example 1:
<pre>
<strong>Input:</strong>
["FreqStack","push","push","push","push","push","push","pop","pop","pop","pop"],
[[],[5],[7],[5],[7],[4],[5],[],[],[],[]]
<strong>Output:</strong> [null,null,null,null,null,null,null,5,7,5,4]
<strong>Explanation:</strong>
After making six .push operations, the stack is [5,7,5,7,4,5] from bottom to top.  Then:

pop() -> returns 5, as 5 is the most frequent.
The stack becomes [5,7,5,7,4].

pop() -> returns 7, as 5 and 7 is the most frequent, but 7 is closest to the top.
The stack becomes [5,7,5,4].

pop() -> returns 5.
The stack becomes [5,7,4].

pop() -> returns 4.
The stack becomes [5,7].
</pre>

#### Note:
* Calls to ```FreqStack.push(int x)``` will be such that ```0 <= x <= 10^9```.
* It is guaranteed that ```FreqStack.pop()``` won't be called if the stack has zero elements.
* The total number of ```FreqStack.push``` calls will not exceed ```10000``` in a single test case.
* The total number of ```FreqStack.pop``` calls will not exceed ```10000``` in a single test case.
* The total number of ```FreqStack.push``` and ```FreqStack.pop``` calls will not exceed ```150000``` across all test cases.

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

struct FreqStack {
    num_freq: HashMap<i32, i32>,
    freq_stack: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self {
            num_freq: HashMap::new(),
            freq_stack: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let freq = self.num_freq.entry(x).or_default();
        *freq += 1;
        self.freq_stack.entry(*freq).or_default().push(x);
        self.max_freq = self.max_freq.max(*freq);
    }

    fn pop(&mut self) -> i32 {
        let stack = self.freq_stack.get_mut(&self.max_freq).unwrap();
        let x = stack.pop().unwrap();
        *self.num_freq.get_mut(&x).unwrap() -= 1;
        if stack.is_empty() {
            self.max_freq -= 1;
        }
        x
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */
```
