# 1604. 警告一小时内使用相同员工卡大于等于三次的人
力扣公司的员工都使用员工卡来开办公室的门。每当一个员工使用一次他的员工卡，安保系统会记录下员工的名字和使用时间。如果一个员工在一小时时间内使用员工卡的次数大于等于三次，这个系统会自动发布一个 **警告** 。

给你字符串数组 `keyName` 和 `keyTime` ，其中 `[keyName[i], keyTime[i]]` 对应一个人的名字和他在 某一天 内使用员工卡的时间。

使用时间的格式是 **24小时制** ，形如 **"HH:MM"** ，比方说 `"23:51"` 和 `"09:49"` 。

请你返回去重后的收到系统警告的员工名字，将它们按 **字典序升序** 排序后返回。

请注意 `"10:00"` - `"11:00"` 视为一个小时时间范围内，而 `"22:51"` - `"23:52"` 不被视为一小时时间范围内。

#### 示例 1:
<pre>
<strong>输入:</strong> keyName = ["daniel","daniel","daniel","luis","luis","luis","luis"], keyTime = ["10:00","10:40","11:00","09:00","11:00","13:00","15:00"]
<strong>输出:</strong> ["daniel"]
<strong>解释:</strong> "daniel" 在一小时内使用了 3 次员工卡（"10:00"，"10:40"，"11:00"）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> keyName = ["alice","alice","alice","bob","bob","bob","bob"], keyTime = ["12:01","12:00","18:00","21:00","21:20","21:30","23:00"]
<strong>输出:</strong> ["bob"]
<strong>解释:</strong> "bob" 在一小时内使用了 3 次员工卡（"21:00"，"21:20"，"21:30"）。
</pre>

#### 提示:
* <code>1 <= keyName.length, keyTime.length <= 10<sup>5</sup></code>
* `keyName.length == keyTime.length`
* `keyTime` 格式为 **"HH:MM"** 。
* 保证 `[keyName[i], keyTime[i]]` 形成的二元对 **互不相同** 。
* `1 <= keyName[i].length <= 10`
* `keyName[i]` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut used_time = HashMap::new();
        let mut ret = BinaryHeap::new();

        for i in 0..key_name.len() {
            let h = key_time[i].get(..2).unwrap().parse::<i32>().unwrap();
            let m = key_time[i].get(3..).unwrap().parse::<i32>().unwrap();

            used_time
                .entry(&key_name[i])
                .or_insert(BinaryHeap::new())
                .push(h * 60 + m);
        }

        for (name, time) in used_time.into_iter() {
            let time = time.into_sorted_vec();

            for i in 2..time.len() {
                if time[i] - time[i - 2] <= 60 {
                    ret.push(name.to_string());
                    break;
                }
            }
        }

        ret.into_sorted_vec()
    }
}
```
