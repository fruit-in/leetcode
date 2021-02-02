# 306. Additive Number
Additive number is a string whose digits can form additive sequence.

A valid additive sequence should contain **at least** three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.

Given a string containing only digits `'0'-'9'`, write a function to determine if it's an additive number.

**Note:** Numbers in the additive sequence **cannot** have leading zeros, so sequence `1, 2, 03` or `1, 02, 3` is invalid.

#### Example 1:
<pre>
<strong>Input:</strong> "112358"
<strong>Output:</strong> true
<strong>Explanation:</strong> The digits can form an additive sequence: 1, 1, 2, 3, 5, 8.
             1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "199100199"
<strong>Output:</strong> true
<strong>Explanation:</strong> The additive sequence is: 1, 99, 100, 199.
             1 + 99 = 100, 99 + 100 = 199
</pre>

#### Constraints:
* `num` consists only of digits `'0'-'9'`.
* `1 <= num.length <= 35`

#### Follow up:
How would you handle overflow for very large input integers?

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} num
# @return {Boolean}
def is_additive_number(num)
  (1...(num.length + 1) / 2).each do |i|
    break if num[0] == '0' && i > 1

    (1..[(num.length - i) / 2, num.length - 2 * i].min).each do |j|
      break if num[i] == '0' && j > 1

      k = i + j
      x = num[0...i].to_i
      y = num[i...k].to_i
      l = k + (x + y).to_s.length

      while l <= num.length
        z = num[k...l].to_i

        break if x + y != z

        x = y
        y = z
        k = l
        l = k + (x + y).to_s.length

        return true if k == num.length
      end
    end
  end

  false
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for i in 1..(num.len() + 1) / 2 {
            if num.get(..1).unwrap() == "0" && i > 1 {
                break;
            }

            for j in 1..=((num.len() - i) / 2).min(num.len() - 2 * i) {
                if num.get(i..i + 1).unwrap() == "0" && j > 1 {
                    break;
                }

                let mut k = i + j;
                let mut x = num.get(..i).unwrap().parse::<u64>().unwrap();
                let mut y = num.get(i..k).unwrap().parse::<u64>().unwrap();
                let mut l = k + (x + y).to_string().len();
                let mut z;

                while l <= num.len() {
                    z = num.get(k..l).unwrap().parse::<u64>().unwrap();

                    if x + y != z {
                        break;
                    }

                    x = y;
                    y = z;
                    k = l;
                    l = k + (x + y).to_string().len();
                }

                if k == num.len() {
                    return true;
                }
            }
        }

        false
    }
}
```
