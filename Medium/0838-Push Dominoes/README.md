# 838. Push Dominoes
There are `N` dominoes in a line, and we place each domino vertically upright.

In the beginning, we simultaneously push some of the dominoes either to the left or to the right.

![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/05/18/domino.png)

After each second, each domino that is falling to the left pushes the adjacent domino on the left.

Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.

When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.

For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.

Given a string "S" representing the initial state. `S[i] = 'L'`, if the i-th domino has been pushed to the left; `S[i] = 'R'`, if the i-th domino has been pushed to the right; `S[i] = '.'`, if the `i`-th domino has not been pushed.

Return a string representing the final state.

#### Example 1:
<pre>
<strong>Input:</strong> ".L.R...LR..L.."
<strong>Output:</strong> "LL.RR.LLRRLL.."
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "RR.L"
<strong>Output:</strong> "RR.L"
<strong>Explanation:</strong> The first domino expends no additional force on the second domino.
</pre>

#### Note:
1. `0 <= N <= 10^5`
2. String `dominoes` contains only `'L'`, `'R'` and `'.'`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} dominoes
# @return {String}
def push_dominoes(dominoes)
  dominoes = ['L'] + dominoes.chars + ['R']
  i = 0

  (1...dominoes.length).each do |j|
    next if dominoes[j] == '.'

    if dominoes[i] == dominoes[j]
      (i...j).each do |k|
        dominoes[k] = dominoes[i]
      end
    elsif dominoes[i] == 'R' && dominoes[j] == 'L'
      (1..(j - i - 1) / 2).each do |k|
        dominoes[i + k] = 'R'
        dominoes[j - k] = 'L'
      end
    end

    i = j
  end

  dominoes[1...-1].join
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = format!("L{}R", dominoes).into_bytes();
        let mut i = 0;

        for j in 1..dominoes.len() {
            if dominoes[j] != b'.' {
                match (dominoes[i], dominoes[j]) {
                    (b'L', b'L') => dominoes[i..j].iter_mut().for_each(|x| *x = b'L'),
                    (b'R', b'R') => dominoes[i..j].iter_mut().for_each(|x| *x = b'R'),
                    (b'R', b'L') => {
                        for k in 1..(j - i + 1) / 2 {
                            dominoes[i + k] = b'R';
                            dominoes[j - k] = b'L';
                        }
                    }
                    _ => (),
                }

                i = j;
            }
        }

        dominoes.remove(0);
        dominoes.pop();

        String::from_utf8(dominoes).unwrap()
    }
}
```
