# 1535. 找出数组游戏的赢家
给你一个由 **不同** 整数组成的整数数组 `arr` 和一个整数 `k` 。

每回合游戏都在数组的前两个元素（即 `arr[0]` 和 `arr[1]` ）之间进行。比较 `arr[0]` 与 `arr[1]` 的大小，较大的整数将会取得这一回合的胜利并保留在位置 `0` ，较小的整数移至数组的末尾。当一个整数赢得 `k` 个连续回合时，游戏结束，该整数就是比赛的 **赢家** 。

返回赢得比赛的整数。

题目数据 **保证** 游戏存在赢家。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,1,3,5,4,6,7], k = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 一起看一下本场游戏每回合的情况：
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/07/30/q-example.png">
因此将进行 4 回合比赛，其中 5 是赢家，因为它连胜 2 回合。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [3,2,1], k = 10
<strong>输出:</strong> 3
<strong>解释:</strong> 3 将会在前 10 个回合中连续获胜。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,9,8,2,3,7,6,4,5], k = 7
<strong>输出:</strong> 9
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [1,11,22,33,44,55,66,77,88,99], k = 1000000000
<strong>输出:</strong> 99
</pre>

#### 提示:
* `2 <= arr.length <= 10^5`
* `1 <= arr[i] <= 10^6`
* `arr` 所含的整数 **各不相同** 。
* `1 <= k <= 10^9`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def get_winner(arr, k)
  winner = arr[0]
  wins = 0

  arr[1..].each do |x|
    if x > winner
      winner = x
      wins = 0
    end
    wins += 1
    break if wins == k
  end

  winner
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut wins = 0;

        for &x in &arr[1..] {
            if x > winner {
                winner = x;
                wins = 0;
            }
            wins += 1;
            if wins == k {
                break;
            }
        }

        winner
    }
}
```
