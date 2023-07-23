# 781. Rabbits in Forest
In a forest, each rabbit has some color. Some subset of rabbits (possibly all of them) tell you how many other rabbits have the same color as them. Those `answers` are placed in an array.

Return the minimum number of rabbits that could be in the forest.

#### Example 1:
<pre>
<strong>Input:</strong> answers = [1, 1, 2]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The two rabbits that answered "1" could both be the same color, say red.
The rabbit than answered "2" can't be red or the answers would be inconsistent.
Say the rabbit that answered "2" was blue.
Then there should be 2 other blue rabbits in the forest that didn't answer into the array.
The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn't.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> answers = [10, 10, 10]
<strong>Output:</strong> 11
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> answers = []
<strong>Output:</strong> 0
</pre>

#### Note:
1. `answers` will have length at most `1000`.
2. Each `answers[i]` will be an integer in the range `[0, 999]`.

## Solutions (Ruby)

### 1. HashMap
```Ruby
# @param {Integer[]} answers
# @return {Integer}
def num_rabbits(answers)
    same_tell = Hash.new(0)
    ret = 0

    for answer in answers
        same_tell[answer + 1] += 1
    end

    same_tell.each do |k, v|
        ret += (v * 1.0 / k).ceil * k
    end

    return ret
end
```

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut same_tell = HashMap::new();

        for answer in answers {
            *same_tell.entry(answer + 1).or_insert(0) += 1;
        }

        same_tell
            .iter()
            .map(|(&k, &v)| (v as f64 / k as f64).ceil() as i32 * k)
            .sum()
    }
}
```
