# 2100. 适合打劫银行的日子
你和一群强盗准备打劫银行。给你一个下标从 **0** 开始的整数数组 `security` ，其中 `security[i]` 是第 `i` 天执勤警卫的数量。日子从 `0` 开始编号。同时给你一个整数 `time` 。

如果第 `i` 天满足以下所有条件，我们称它为一个适合打劫银行的日子：

* 第 `i` 天前和后都分别至少有 `time` 天。
* 第 `i` 天前连续 `time` 天警卫数目都是非递增的。
* 第 `i` 天后连续 `time` 天警卫数目都是非递减的。

更正式的，第 `i` 天是一个合适打劫银行的日子当且仅当：`security[i - time] >= security[i - time + 1] >= ... >= security[i] <= ... <= security[i + time - 1] <= security[i + time]`.

请你返回一个数组，包含 **所有** 适合打劫银行的日子（下标从 **0** 开始）。返回的日子可以 **任意** 顺序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> security = [5,3,3,3,5,6,2], time = 2
<strong>输出:</strong> [2,3]
<strong>解释:</strong>
第 2 天，我们有 security[0] >= security[1] >= security[2] <= security[3] <= security[4] 。
第 3 天，我们有 security[1] >= security[2] >= security[3] <= security[4] <= security[5] 。
没有其他日子符合这个条件，所以日子 2 和 3 是适合打劫银行的日子。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> security = [1,1,1,1,1], time = 0
<strong>输出:</strong> [0,1,2,3,4]
<strong>解释:</strong>
因为 time 等于 0 ，所以每一天都是适合打劫银行的日子，所以返回每一天。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> security = [1,2,3,4,5,6], time = 2
<strong>输出:</strong> []
<strong>解释:</strong>
没有任何一天的前 2 天警卫数目是非递增的。
所以没有适合打劫银行的日子，返回空数组。
</pre>

#### 提示:
* <code>1 <= security.length <= 10<sup>5</sup></code>
* <code>0 <= security[i], time <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let mut prefix_l = vec![0; security.len()];
        let mut prefix_r = vec![0; security.len()];

        for i in 1..security.len() {
            if security[i] <= security[i - 1] {
                prefix_l[i] = prefix_l[i - 1] + 1;
            }
            if security[security.len() - i - 1] <= security[security.len() - i] {
                prefix_r[security.len() - i - 1] = prefix_r[security.len() - i] + 1;
            }
        }

        (time..security.len() as i32 - time)
            .filter(|&i| prefix_l[i as usize] >= time && prefix_r[i as usize] >= time)
            .collect()
    }
}
```
