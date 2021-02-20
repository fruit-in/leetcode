# 187. Repeated DNA Sequences
All DNA is composed of a series of nucleotides abbreviated as `'A'`, `'C'`, `'G'`, and `'T'`, for example: `"ACGAATTCCG"`. When studying DNA, it is sometimes useful to identify repeated sequences within the DNA.

Write a function to find all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.

#### Example 1:
<pre>
<strong>Input:</strong> s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
<strong>Output:</strong> ["AAAAACCCCC","CCCCCAAAAA"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "AAAAAAAAAAAAA"
<strong>Output:</strong> ["AAAAAAAAAA"]
</pre>

#### Constraints:
* <code>0 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is `'A'`, `'C'`, `'G'`, or `'T'`.

## Solutions (Ruby)

### 1. HashMap
```Ruby
# @param {String} s
# @return {String[]}
def find_repeated_dna_sequences(s)
  counter = {}
  counter.default = 0

  (0..s.length - 10).each do |i|
    counter[s[i...i + 10]] += 1
  end

  counter.filter { |_, c| c > 1 }.keys
end
```

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;
use std::str;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut counter = HashMap::new();

        for w in s.windows(10) {
            *counter.entry(str::from_utf8(w).unwrap()).or_insert(0) += 1;
        }

        counter
            .into_iter()
            .filter(|(_, c)| *c > 1)
            .map(|(s, _)| s.to_string())
            .collect()
    }
}
```
