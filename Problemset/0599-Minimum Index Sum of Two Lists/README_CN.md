# 599. 两个列表的最小索引总和
假设Andy和Doris想在晚餐时选择一家餐厅，并且他们都有一个表示最喜爱餐厅的列表，每个餐厅的名字用字符串表示。

你需要帮助他们用**最少的索引和**找出他们**共同喜爱的餐厅**。 如果答案不止一个，则输出所有答案并且不考虑顺序。 你可以假设总是存在一个答案。

#### 示例 1:
<pre>
<strong>输入:</strong>
["Shogun", "Tapioca Express", "Burger King", "KFC"]
["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]
<strong>输出:</strong> ["Shogun"]
<strong>解释:</strong> 他们唯一共同喜爱的餐厅是“Shogun”。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
["Shogun", "Tapioca Express", "Burger King", "KFC"]
["KFC", "Shogun", "Burger King"]
<strong>输出:</strong> ["Shogun"]
<strong>解释:</strong> 他们共同喜爱且具有最小索引和的餐厅是“Shogun”，它有最小的索引和1(0+1)。
</pre>

#### 提示:
1. 两个列表的长度范围都在 [1, 1000]内。
2. 两个列表中的字符串的长度将在[1，30]的范围内。
3. 下标从0开始，到列表的长度减1。
4. 两个列表都没有重复的元素。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min_sum = std::usize::MAX;
        let mut ret = Vec::new();

        for i in 0..list1.len() {
            for j in 0..list2.len() {
                if list1[i] == list2[j] {
                    if i + j == min_sum {
                        ret.push(list2[j].to_string());
                    } else if i + j < min_sum {
                        ret.clear();
                        ret.push(list2[j].to_string());
                        min_sum = i + j;
                    }
                    break;
                }
            }
        }

        ret
    }
}
```

### 2. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut rest_idx = list1
            .iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<&String, usize>>();
        let mut min_sum = std::usize::MAX;
        let mut ret = Vec::new();

        for i in 0..list2.len() {
            if let Some(j) = rest_idx.get(&list2[i]) {
                if i + j == min_sum {
                    ret.push(list2[i].to_string());
                } else if i + j < min_sum {
                    ret.clear();
                    ret.push(list2[i].to_string());
                    min_sum = i + j;
                }
            }
        }

        ret
    }
}
```

### 3. 遍历索引和
```Rust
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut flag = false;
        let mut ret = Vec::new();

        for sum in 0..(list1.len() + list2.len() - 1) {
            for i in (sum + 1).saturating_sub(list2.len())..(sum + 1).min(list1.len()) {
                if list1[i] == list2[sum - i] {
                    flag = true;
                    ret.push(list1[i].to_string());
                }
            }

            if flag {
                break;
            }
        }

        ret
    }
}
```
