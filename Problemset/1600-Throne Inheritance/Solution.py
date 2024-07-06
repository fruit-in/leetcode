class ThroneInheritance:

    def __init__(self, kingName: str):
        self.king = kingName
        self.living = {kingName}
        self.children = {kingName: []}

    def birth(self, parentName: str, childName: str) -> None:
        self.living.add(childName)
        self.children[parentName].append(childName)
        self.children[childName] = []

    def death(self, name: str) -> None:
        self.living.remove(name)

    def getInheritanceOrder(self) -> List[str]:
        people = [self.king]
        ret = []

        while people != []:
            person = people.pop()
            if person in self.living:
                ret.append(person)
            for child in self.children[person][::-1]:
                people.append(child)

        return ret


# Your ThroneInheritance object will be instantiated and called as such:
# obj = ThroneInheritance(kingName)
# obj.birth(parentName,childName)
# obj.death(name)
# param_3 = obj.getInheritanceOrder()
