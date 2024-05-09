# 1178. Number of Valid Words for Each Puzzle
With respect to a given `puzzle` string, a `word` is *valid* if both the following conditions are satisfied:

* `word` contains the first letter of `puzzle`.
* For each letter in `word`, that letter is in `puzzle`.
    * For example, if the puzzle is `"abcdefg"`, then valid words are `"faced"`, `"cabbage"`, and `"baggage"`, while
    * invalid words are `"beefed"` (does not include `'a'`) and `"based"` (includes `'s'` which is not in the puzzle).

Return *an array* `answer`, *where* `answer[i]` *is the number of words in the given word list* `words` *that is valid with respect to the puzzle* `puzzles[i]`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["aaaa","asas","able","ability","actt","actor","access"], puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
<strong>Output:</strong> [1,1,3,2,4,0]
<strong>Explanation:</strong>
1 valid word for "aboveyz" : "aaaa"
1 valid word for "abrodyz" : "aaaa"
3 valid words for "abslute" : "aaaa", "asas", "able"
2 valid words for "absoryz" : "aaaa", "asas"
4 valid words for "actresz" : "aaaa", "asas", "actt", "access"
There are no valid words for "gaswxyz" cause none of the words in the list contains letter 'g'.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["apple","pleas","please"], puzzles = ["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"]
<strong>Output:</strong> [0,1,3,2,0]
</pre>

#### Constraints:
* <code>1 <= words.length <= 10<sup>5</sup></code>
* `4 <= words[i].length <= 50`
* <code>1 <= puzzles.length <= 10<sup>4</sup></code>
* `puzzles[i].length == 7`
* `words[i]` and `puzzles[i]` consist of lowercase English letters.
* Each `puzzles[i]` does not contain repeated characters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut answer = vec![0; puzzles.len()];

        for word in words {
            let bits = word.bytes().fold(0_i32, |b, c| b | (1 << (c - b'a')));

            if bits.count_ones() < 8 {
                *count.entry(bits).or_insert(0) += 1;
            }
        }

        for i in 0..answer.len() {
            let puzzle = puzzles[i].as_bytes();
            let mut combins = vec![1 << (puzzle[0] - b'a')];
            answer[i] += count.get(&combins[0]).unwrap_or(&0);

            for j in 1..puzzle.len() {
                for k in 0..combins.len() {
                    let combin = combins[k] | (1 << (puzzle[j] - b'a'));
                    combins.push(combin);
                    answer[i] += count.get(&combin).unwrap_or(&0);
                }
            }
        }

        answer
    }
}
```
