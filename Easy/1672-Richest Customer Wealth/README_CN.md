# 1672. 最富有客户的资产总量
给你一个 `m x n` 的整数网格 `accounts` ，其中 `accounts[i][j]` 是第 `i` 位客户在第 `j` 家银行托管的资产数量。返回最富有客户所拥有的 **资产总量** 。

客户的 **资产总量** 就是他们在各家银行托管的资产数量之和。最富有客户就是 **资产总量** 最大的客户。

#### 示例 1:
<pre>
<strong>输入:</strong> accounts = [[1,2,3],[3,2,1]]
<strong>输出:</strong> 6
<strong>解释:</strong>
第 1 位客户的资产总量 = 1 + 2 + 3 = 6
第 2 位客户的资产总量 = 3 + 2 + 1 = 6
两位客户都是最富有的，资产总量都是 6 ，所以返回 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> accounts = [[1,5],[7,3],[3,5]]
<strong>输出:</strong> 10
<strong>解释:</strong>
第 1 位客户的资产总量 = 6
第 2 位客户的资产总量 = 10
第 3 位客户的资产总量 = 8
第 2 位客户是最富有的，资产总量是 10
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> accounts = [[2,8,7],[7,1,3],[1,9,5]]
<strong>输出:</strong> 17
</pre>

#### 提示:
* `m == accounts.length`
* `n == accounts[i].length`
* `1 <= m, n <= 50`
* `1 <= accounts[i][j] <= 100`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} accounts
# @return {Integer}
def maximum_wealth(accounts)
  accounts.map { |account| account.sum }.max
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|account| account.iter().sum())
            .max()
            .unwrap()
    }
}
```
