# 470. Implement Rand10() Using Rand7()
Given a function ```rand7``` which generates a uniform random integer in the range 1 to 7, write a function ```rand10``` which generates a uniform random integer in the range 1 to 10.

Do NOT use system's ```Math.random()```.

#### Example 1:
<pre>
<strong>Input:</strong> 1
<strong>Output:</strong> [7]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> [8,4]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> [8,1,10]
</pre>

#### Note:
1. ```rand7``` is predefined.
2. Each testcase has one argument: ```n```, the number of times that ```rand10``` is called.

#### Follow up:
1. What is the [expected value](https://en.wikipedia.org/wiki/Expected_value) for the number of calls to ```rand7()``` function?
2. Could you minimize the number of calls to ```rand7()```?

## Solutions (Rust)

### 1. Solution
```Rust
/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut a = rand7();
        let mut b = rand7() * 7;

        while a + b > 47 {
            a = rand7();
            b = rand7() * 7;
        }

        (a + b + 2) % 10 + 1
    }
}
```
