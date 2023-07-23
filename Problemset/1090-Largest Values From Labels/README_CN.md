# 1090. 受标签影响的最大值
我们有一个项的集合，其中第 `i` 项的值为 `values[i]`，标签为 `labels[i]`。

我们从这些项中选出一个子集 `S`，这样一来：
* `|S| <= num_wanted`
* 对于任意的标签 `L`，子集 `S` 中标签为 `L` 的项的数目总满足 `<= use_limit`。

返回子集 `S` 的最大可能的 **和**。

#### 示例 1:
<pre>
<strong>输入:</strong> values = [5,4,3,2,1], labels = [1,1,2,2,3], num_wanted = 3, use_limit = 1
<strong>输出:</strong> 9
<strong>解释:</strong> 选出的子集是第一项，第三项和第五项。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> values = [5,4,3,2,1], labels = [1,3,3,3,2], num_wanted = 3, use_limit = 2
<strong>输出:</strong> 12
<strong>解释:</strong> 选出的子集是第一项，第二项和第三项。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> values = [9,8,8,7,6], labels = [0,0,0,1,1], num_wanted = 3, use_limit = 1
<strong>输出:</strong> 16
<strong>解释:</strong> 选出的子集是第一项和第四项。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> values = [9,8,8,7,6], labels = [0,0,0,1,1], num_wanted = 3, use_limit = 2
<strong>输出:</strong> 24
<strong>解释:</strong> 选出的子集是第一项，第二项和第四项。
</pre>

#### 提示:
1. `1 <= values.length == labels.length <= 20000`
2. `0 <= values[i], labels[i] <= 20000`
3. `1 <= num_wanted, use_limit <= values.length`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} values
# @param {Integer[]} labels
# @param {Integer} num_wanted
# @param {Integer} use_limit
# @return {Integer}
def largest_vals_from_labels(values, labels, num_wanted, use_limit)
    items = Array.new(values.length) {|i| [values[i], labels[i]] }
    items.sort! {|a, b| b <=> a }
    label_use = Hash.new(0)
    cnt = 0
    ret = 0

    for item in items
        if label_use[item[1]] < use_limit
            ret += item[0]
            cnt += 1
            label_use[item[1]] += 1
            break if cnt >= num_wanted
        end
    end

    return ret
end
```
