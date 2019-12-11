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
