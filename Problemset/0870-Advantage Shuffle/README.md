# 870. Advantage Shuffle
Given two arrays `A` and `B` of equal size, the *advantage of `A` with respect to `B`* is the number of indices `i` for which `A[i] > B[i]`.

Return **any** permutation of `A` that maximizes its advantage with respect to `B`.

#### Example 1:
<pre>
<strong>Input:</strong> A = [2,7,11,15], B = [1,10,4,11]
<strong>Output:</strong> [2,11,7,15]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> A = [12,24,8,32], B = [13,25,32,11]
<strong>Output:</strong> [24,32,8,12]
</pre>

#### Note:
1. `1 <= A.length = B.length <= 10000`
2. `0 <= A[i] <= 10^9`
3. `0 <= B[i] <= 10^9`

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer[]} a
# @param {Integer[]} b
# @return {Integer[]}
def advantage_count(a, b)
  i = 0
  j = b.length - 1
  indices = (0...b.length).to_a
  ret = Array.new(b.length)

  a.sort!
  indices.sort_by! { |k| -b[k] }

  indices.each do |k|
    if b[k] < a[j]
      ret[k] = a[j]
      j -= 1
    else
      ret[k] = a[i]
      i += 1
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = b.len() - 1;
        let mut indices = (0..b.len()).collect::<Vec<_>>();
        let mut ret = vec![0; b.len()];

        a.sort_unstable();
        indices.sort_unstable_by_key(|&k| -b[k]);

        for k in indices {
            if b[k] < a[j] {
                ret[k] = a[j];
                j -= 1;
            } else {
                ret[k] = a[i];
                i += 1;
            }
        }

        ret
    }
}
```
