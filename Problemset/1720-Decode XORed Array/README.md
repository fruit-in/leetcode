# 1720. Decode XORed Array
There is a **hidden** integer array `arr` that consists of `n` non-negative integers.

It was encoded into another integer array `encoded` of length `n - 1`, such that `encoded[i] = arr[i] XOR arr[i + 1]`. For example, if `arr = [1,0,2,1]`, then `encoded = [1,2,3]`.

You are given the `encoded` array. You are also given an integer `first`, that is the first element of `arr`, i.e. `arr[0]`.

Return *the original array* `arr`. It can be proved that the answer exists and is unique.

#### Example 1:
<pre>
<strong>Input:</strong> encoded = [1,2,3], first = 1
<strong>Output:</strong> [1,0,2,1]
<strong>Explanation:</strong> If arr = [1,0,2,1], then first = 1 and encoded = [1 XOR 0, 0 XOR 2, 2 XOR 1] = [1,2,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> encoded = [6,2,7,3], first = 4
<strong>Output:</strong> [4,2,0,7,4]
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>4</sup></code>
* `encoded.length == n - 1`
* <code>0 <= encoded[i] <= 10<sup>5</sup></code>
* <code>0 <= first <= 10<sup>5</sup></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} encoded
# @param {Integer} first
# @return {Integer[]}
def decode(encoded, first)
  ret = [first]

  encoded.each do |n|
    first ^= n
    ret.push(first)
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn decode(encoded: Vec<i32>, mut first: i32) -> Vec<i32> {
        let mut ret = vec![first];

        for n in encoded {
            first ^= n;
            ret.push(first);
        }

        ret
    }
}
```
