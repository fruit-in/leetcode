# 2115. 从给定原材料中找到所有可以做出的菜
你有 `n` 道不同菜的信息。给你一个字符串数组 `recipes` 和一个二维字符串数组 `ingredients` 。第 `i` 道菜的名字为 `recipes[i]` ，如果你有它 **所有** 的原材料 `ingredients[i]` ，那么你可以 **做出** 这道菜。一道菜的原材料可能是 **另一道** 菜，也就是说 `ingredients[i]` 可能包含 `recipes` 中另一个字符串。

同时给你一个字符串数组 `supplies` ，它包含你初始时拥有的所有原材料，每一种原材料你都有无限多。

请你返回你可以做出的所有菜。你可以以 **任意顺序** 返回它们。

注意两道菜在它们的原材料中可能互相包含。

#### 示例 1:
<pre>
<strong>输入:</strong> recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
<strong>输出:</strong> ["bread"]
<strong>解释:</strong>
我们可以做出 "bread" ，因为我们有原材料 "yeast" 和 "flour" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
<strong>输出:</strong> ["bread","sandwich"]
<strong>解释:</strong>
我们可以做出 "bread" ，因为我们有原材料 "yeast" 和 "flour" 。
我们可以做出 "sandwich" ，因为我们有原材料 "meat" 且可以做出原材料 "bread" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
<strong>输出:</strong> ["bread","sandwich","burger"]
<strong>解释:</strong>
我们可以做出 "bread" ，因为我们有原材料 "yeast" 和 "flour" 。
我们可以做出 "sandwich" ，因为我们有原材料 "meat" 且可以做出原材料 "bread" 。
我们可以做出 "burger" ，因为我们有原材料 "meat" 且可以做出原材料 "bread" 和 "sandwich" 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast"]
<strong>输出:</strong> []
<strong>解释:</strong>
我们没法做出任何菜，因为我们只有原材料 "yeast" 。
</pre>

#### 提示:
* `n == recipes.length == ingredients.length`
* `1 <= n <= 100`
* `1 <= ingredients[i].length, supplies.length <= 100`
* `1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10`
* `recipes[i]`, `ingredients[i][j]` 和 `supplies[k]` 只包含小写英文字母。
* 所有 `recipes` 和 `supplies` 中的值互不相同。
* `ingredients[i]` 中的字符串互不相同。

## 题解 (Python)

### 1. 题解
```Python
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
```
