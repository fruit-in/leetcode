# 2115. Find All Possible Recipes from Given Supplies
You have information about `n` different recipes. You are given a string array `recipes` and a 2D string array `ingredients`. The <code>i<sup>th</sup></code> recipe has the name `recipes[i]`, and you can **create** it if you have **all** the needed ingredients from `ingredients[i]`. Ingredients to a recipe may need to be created from **other** recipes, i.e., `ingredients[i]` may contain a string that is in `recipes`.

You are also given a string array `supplies` containing all the ingredients that you initially have, and you have an infinite supply of all of them.

Return *a list of all the recipes that you can create*. You may return the answer in **any order**.

Note that two recipes may contain each other in their ingredients.

#### Example 1:
<pre>
<strong>Input:</strong> recipes = ["bread"], ingredients = [["yeast","flour"]], supplies = ["yeast","flour","corn"]
<strong>Output:</strong> ["bread"]
<strong>Explanation:</strong>
We can create "bread" since we have the ingredients "yeast" and "flour".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> recipes = ["bread","sandwich"], ingredients = [["yeast","flour"],["bread","meat"]], supplies = ["yeast","flour","meat"]
<strong>Output:</strong> ["bread","sandwich"]
<strong>Explanation:</strong>
We can create "bread" since we have the ingredients "yeast" and "flour".
We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> recipes = ["bread","sandwich","burger"], ingredients = [["yeast","flour"],["bread","meat"],["sandwich","meat","bread"]], supplies = ["yeast","flour","meat"]
<strong>Output:</strong> ["bread","sandwich","burger"]
<strong>Explanation:</strong>
We can create "bread" since we have the ingredients "yeast" and "flour".
We can create "sandwich" since we have the ingredient "meat" and can create the ingredient "bread".
We can create "burger" since we have the ingredient "meat" and can create the ingredients "bread" and "sandwich".
</pre>

#### Constraints:
* `n == recipes.length == ingredients.length`
* `1 <= n <= 100`
* `1 <= ingredients[i].length, supplies.length <= 100`
* `1 <= recipes[i].length, ingredients[i][j].length, supplies[k].length <= 10`
* `recipes[i]`, `ingredients[i][j]`, and `supplies[k]` consist only of lowercase English letters.
* All the values of `recipes` and `supplies` combined are unique.
* Each `ingredients[i]` does not contain any duplicate values.

## Solutions (Python)

### 1. Solution
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
