# 599. Minimum Index Sum of Two Lists
Suppose Andy and Doris want to choose a restaurant for dinner, and they both have a list of favorite restaurants represented by strings.

You need to help them find out their **common interest** with the **least list index sum**. If there is a choice tie between answers, output all of them with no order requirement. You could assume there always exists an answer.

#### Example 1:
<pre>
<strong>Input:</strong>
["Shogun", "Tapioca Express", "Burger King", "KFC"]
["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]
<strong>Output:</strong> ["Shogun"]
<strong>Explanation:</strong> The only restaurant they both like is "Shogun".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
["Shogun", "Tapioca Express", "Burger King", "KFC"]
["KFC", "Shogun", "Burger King"]
<strong>Output:</strong> ["Shogun"]
<strong>Explanation:</strong> The restaurant they both like and have the least index sum is "Shogun" with index sum 1 (0+1).
</pre>

#### Note:
1. The length of both lists will be in the range of [1, 1000].
2. The length of strings in both lists will be in the range of [1, 30].
3. The index is starting from 0 to the list length minus 1.
4. No duplicates in both lists.

## Solutions (Rust)

### 1. Brute Force
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

### 2. HashMap
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

### 3. Increasing Sum
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
