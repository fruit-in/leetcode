# 893. Groups of Special-Equivalent Strings
You are given an array ```A``` of strings.

A *move onto ```S```* consists of swapping any two even indexed characters of ```S```, or any two odd indexed characters of ```S```.

Two strings ```S``` and ```T``` are *special-equivalent* if after any number of *moves onto ```S```*, ```S == T```.

For example, ```S = "zzxy"``` and ```T = "xyzz"``` are special-equivalent because we may make the moves ```"zzxy" -> "xzzy" -> "xyzz"``` that swap ```S[0]``` and ```S[2]```, then ```S[1]``` and ```S[3]```.

Now, a *group of special-equivalent strings from ```A```* is a non-empty subset of A such that:

1. Every pair of strings in the group are special equivalent, and;
2. The group is the largest size possible (ie., there isn't a string S not in the group such that S is special equivalent to every string in the group)

Return the number of groups of special-equivalent strings from ```A```.

#### Example 1:
<pre>
<strong>Input:</strong> ["abcd","cdab","cbad","xyzz","zzxy","zzyx"]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
One group is ["abcd", "cdab", "cbad"], since they are all pairwise special equivalent, and none of the other strings are all pairwise special equivalent to these.

The other two groups are ["xyzz", "zzxy"] and ["zzyx"].  Note that in particular, "zzxy" is not special equivalent to "zzyx".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ["abc","acb","bac","bca","cab","cba"]
<strong>Output:</strong> 3
</pre>

#### Note:
* ```1 <= A.length <= 1000```
* ```1 <= A[i].length <= 20```
* All ```A[i]``` have the same length.
* All ```A[i]``` consist of only lowercase letters.

## Solutions (Rust)

### 1. Count
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut set = HashSet::new();

        for s in a {
            let mut cnt = vec![0; 52];
            for (i, c) in s.bytes().enumerate() {
                cnt[(c - b'a') as usize + 26 * (i % 2)] += 1;
            }
            set.insert(cnt);
        }

        set.len() as i32
    }
}
```
