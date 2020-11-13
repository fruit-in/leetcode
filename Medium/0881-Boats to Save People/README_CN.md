# 881. 救生艇
第 `i` 个人的体重为 `people[i]`，每艘船可以承载的最大重量为 `limit`。

每艘船最多可同时载两人，但条件是这些人的重量之和最多为 `limit`。

返回载到每一个人所需的最小船数。(保证每个人都能被船载)。

#### 示例 1:
<pre>
<b>输入:</b> people = [1,2], limit = 3
<b>输出:</b> 1
<b>解释:</b> 1 艘船载 (1, 2)
</pre>

#### 示例 2:
<pre>
<b>输入:</b> people = [3,2,2,1], limit = 3
<b>输出:</b> 3
<b>解释:</b> 3 艘船分别载 (1, 2), (2) 和 (3)
</pre>

#### 示例 3:
<pre>
<b>输入:</b> people = [3,5,3,4], limit = 5
<b>输出:</b> 4
<b>解释:</b> 4 艘船分别载 (3), (3), (4), (5)
</pre>

#### 提示:
* `1 <= people.length <= 50000`
* `1 <= people[i] <= limit <= 30000`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} people
# @param {Integer} limit
# @return {Integer}
def num_rescue_boats(people, limit)
    people.sort!
    i, j = 0, people.length - 1
    ret = 0

    while i <= j
        ret += 1
        i += 1 if people[j] + people[i] <= limit
        j -= 1
    end

    return ret
end
```
