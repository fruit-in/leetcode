# 690. 员工的重要性
给定一个保存员工信息的数据结构，它包含了员工**唯一的id**，**重要度** 和 **直系下属的id**。

比如，员工1是员工2的领导，员工2是员工3的领导。他们相应的重要度为15, 10, 5。那么员工1的数据结构是[1, 15, [2]]，员工2的数据结构是[2, 10, [3]]，员工3的数据结构是[3, 5, []]。注意虽然员工3也是员工1的一个下属，但是由于**并不是直系**下属，因此没有体现在员工1的数据结构中。

现在输入一个公司的所有员工信息，以及单个员工id，返回这个员工和他所有下属的重要度之和。

#### 示例 1:
<pre>
<strong>输入:</strong> [[1, 5, [2, 3]], [2, 3, []], [3, 3, []]], 1
<strong>输出:</strong> 11
<strong>解释:</strong>
员工1自身的重要度是5，他有两个直系下属2和3，而且2和3的重要度均为3。因此员工1的总重要度是 5 + 3 + 3 = 11。
</pre>

#### 注意:
1. 一个员工最多有一个**直系**领导，但是可以有多个**直系**下属
2. 员工数量不超过2000。

## 题解 (Python)

### 1. 广度优先搜索
```Python3
"""
# Employee info
class Employee:
    def __init__(self, id: int, importance: int, subordinates: List[int]):
        # It's the unique id of each node.
        # unique id of this employee
        self.id = id
        # the importance value of this employee
        self.importance = importance
        # the id of direct subordinates
        self.subordinates = subordinates
"""
class Solution:
    def getImportance(self, employees: List['Employee'], id: int) -> int:
        id_employee = {employee.id: employee for employee in employees}

        total = 0
        subs = [id]

        while subs:
            temp = []
            for id in subs:
                total += id_employee[id].importance
                temp.extend(id_employee[id].subordinates)
            subs = temp

        return total
```
