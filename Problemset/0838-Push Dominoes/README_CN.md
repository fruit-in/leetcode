# 838. 推多米诺
一行中有 `N` 张多米诺骨牌，我们将每张多米诺骨牌垂直竖立。

在开始时，我们同时把一些多米诺骨牌向左或向右推。

![](https://aliyun-lc-upload.oss-cn-hangzhou.aliyuncs.com/aliyun-lc-upload/uploads/2018/05/19/domino.png)

每过一秒，倒向左边的多米诺骨牌会推动其左侧相邻的多米诺骨牌。

同样地，倒向右边的多米诺骨牌也会推动竖立在其右侧的相邻多米诺骨牌。

如果同时有多米诺骨牌落在一张垂直竖立的多米诺骨牌的两边，由于受力平衡， 该骨牌仍然保持不变。

就这个问题而言，我们会认为正在下降的多米诺骨牌不会对其它正在下降或已经下降的多米诺骨牌施加额外的力。

给定表示初始状态的字符串 "S" 。如果第 i 张多米诺骨牌被推向左边，则 `S[i] = 'L'`；如果第 i 张多米诺骨牌被推向右边，则 `S[i] = 'R'`；如果第 i 张多米诺骨牌没有被推动，则 `S[i] = '.'`。

返回表示最终状态的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> ".L.R...LR..L.."
<strong>输出:</strong> "LL.RR.LLRRLL.."
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "RR.L"
<strong>输出:</strong> "RR.L"
<strong>说明:</strong> 第一张多米诺骨牌没有给第二张施加额外的力。
</pre>

#### 提示:
1. `0 <= N <= 10^5`
2. 表示多米诺骨牌状态的字符串只含有 `'L'`，`'R'`; 以及 `'.'`;

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
