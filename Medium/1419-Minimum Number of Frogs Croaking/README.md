# 1419. Minimum Number of Frogs Croaking
Given the string `croakOfFrogs`, which represents a combination of the string "croak" from different frogs, that is, multiple frogs can croak at the same time, so multiple “croak” are mixed. *Return the minimum number of* different *frogs to finish all the croak in the given string*.

A valid "croak" means a frog is printing 5 letters ‘c’, ’r’, ’o’, ’a’, ’k’ **sequentially**. The frogs have to print all five letters to finish a croak. If the given string is not a combination of valid "croak" return -1.

#### Example 1:
<pre>
<strong>Input:</strong> croakOfFrogs = "croakcroak"
<strong>Output:</strong> 1
<strong>Explanation:</strong> One frog yelling "croak" twice.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> croakOfFrogs = "crcoakroak"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The minimum number of frogs is two.
The first frog could yell "<b>cr</b>c<b>oak</b>roak".
The second frog could yell later "cr<b>c</b>oak<b>roak</b>".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> croakOfFrogs = "croakcrook"
<strong>Output:</strong> -1
<strong>Explanation:</strong> The given string is an invalid combination of "croak" from different frogs.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> croakOfFrogs = "croakcroa"
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `1 <= croakOfFrogs.length <= 10^5`
* All characters in the string are: `'c'`, `'r'`, `'o'`, `'a'` or `'k'`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} croak_of_frogs
# @return {Integer}
def min_number_of_frogs(croak_of_frogs)
  counter = { 'c' => 0, 'r' => 0, 'o' => 0, 'a' => 0 }
  prev = { 'r' => 'c', 'o' => 'r', 'a' => 'o' }
  frogs = 0
  ret = 0

  croak_of_frogs.each_char do |c|
    if c == 'c'
      counter['c'] += 1
      frogs += 1
      ret = [ret, frogs].max
    elsif c == 'k'
      return -1 if counter['a'] == 0

      counter['a'] -= 1
      frogs -= 1
    else
      return -1 if counter[prev[c]] == 0

      counter[prev[c]] -= 1
      counter[c] += 1
    end
  end

  counter.values.all? { |v| v == 0 } ? ret : -1
end
```

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut counter = vec![('c', 0), ('r', 0), ('o', 0), ('a', 0)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut prev = vec![('r', 'c'), ('o', 'r'), ('a', 'o')]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut frogs = 0;
        let mut ret = 0;

        for c in croak_of_frogs.chars() {
            match c {
                'c' => {
                    *counter.get_mut(&'c').unwrap() += 1;
                    frogs += 1;
                    ret = ret.max(frogs);
                }
                'k' => {
                    if counter[&'a'] == 0 {
                        return -1;
                    }
                    *counter.get_mut(&'a').unwrap() -= 1;
                    frogs -= 1;
                }
                _ => {
                    if counter[&prev[&c]] == 0 {
                        return -1;
                    }
                    *counter.get_mut(&prev[&c]).unwrap() -= 1;
                    *counter.get_mut(&c).unwrap() += 1;
                }
            }
        }

        if counter.values().all(|&v| v == 0) {
            ret
        } else {
            -1
        }
    }
}
```
