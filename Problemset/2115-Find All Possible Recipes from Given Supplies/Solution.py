class Solution:
    def findAllRecipes(self, recipes: List[str], ingredients: List[List[str]], supplies: List[str]) -> List[str]:
        n = len(recipes)
        needed = {}
        indegrees = [0] * n
        ret = []

        for i in range(n):
            for ingredient in ingredients[i]:
                if ingredient not in needed:
                    needed[ingredient] = []
                needed[ingredient].append(i)
                indegrees[i] += 1

        while supplies != []:
            for i in needed.get(supplies.pop(), []):
                indegrees[i] -= 1
                if indegrees[i] == 0:
                    supplies.append(recipes[i])
                    ret.append(recipes[i])

        return ret
