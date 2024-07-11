# 1418. Display Table of Food Orders in a Restaurant
Given the array `orders`, which represents the orders that customers have done in a restaurant. More specifically <code>orders[i]=[customerName<sub>i</sub>,tableNumber<sub>i</sub>,foodItem<sub>i</sub>]</code> where <code>customerName<sub>i</sub></code> is the name of the customer, <code>tableNumber<sub>i</sub></code> is the table customer sit at, and <code>foodItem<sub>i</sub></code> is the item customer orders.

*Return the restaurant's “**display table**”*. The “**display table**” is a table whose row entries denote how many of each food item each table ordered. The first column is the table number and the remaining columns correspond to each food item in alphabetical order. The first row should be a header whose first column is “Table”, followed by the names of the food items. Note that the customer names are not part of the table. Additionally, the rows should be sorted in numerically increasing order.

#### Example 1:
<pre>
<strong>Input:</strong> orders = [["David","3","Ceviche"],["Corina","10","Beef Burrito"],["David","3","Fried Chicken"],["Carla","5","Water"],["Carla","5","Ceviche"],["Rous","3","Ceviche"]]
<strong>Output:</strong> [["Table","Beef Burrito","Ceviche","Fried Chicken","Water"],["3","0","2","1","0"],["5","0","1","0","1"],["10","1","0","0","0"]]
<strong>Explanation:</strong>
The displaying table looks like:
Table,Beef Burrito,Ceviche,Fried Chicken,Water
3    ,0           ,2      ,1            ,0
5    ,0           ,1      ,0            ,1
10   ,1           ,0      ,0            ,0
For the table 3: David orders "Ceviche" and "Fried Chicken", and Rous orders "Ceviche".
For the table 5: Carla orders "Water" and "Ceviche".
For the table 10: Corina orders "Beef Burrito".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> orders = [["James","12","Fried Chicken"],["Ratesh","12","Fried Chicken"],["Amadeus","12","Fried Chicken"],["Adam","1","Canadian Waffles"],["Brianna","1","Canadian Waffles"]]
<strong>Output:</strong> [["Table","Canadian Waffles","Fried Chicken"],["1","2","0"],["12","0","3"]]
<strong>Explanation:</strong>
For the table 1: Adam and Brianna order "Canadian Waffles".
For the table 12: James, Ratesh and Amadeus order "Fried Chicken".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> orders = [["Laura","2","Bean Burrito"],["Jhon","2","Beef Burrito"],["Melissa","2","Soda"]]
<strong>Output:</strong> [["Table","Bean Burrito","Beef Burrito","Soda"],["2","1","1","1"]]
</pre>

#### Constraints:
* `1 <= orders.length <= 5 * 10^4`
* `orders[i].length == 3`
* <code>1 <= customerName<sub>i</sub>.length, foodItem<sub>i</sub>.length <= 20</code>
* <code>customerName<sub>i</sub></code> and <code>foodItem<sub>i</sub></code> consist of lowercase and uppercase English letters and the space character.
* <code>tableNumber<sub>i</sub></code> is a valid integer between `1` and `500`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def displayTable(self, orders: List[List[str]]) -> List[List[str]]:
        tables = set()
        foods = set()
        count = {}
        ret = []

        for _, tn, fi in orders:
            tables.add(tn)
            foods.add(fi)
            count[(tn, fi)] = count.get((tn, fi), 0) + 1

        ret.append(["Table"] + sorted(foods))

        for tn in sorted(tables, key=int):
            ret.append([tn])
            for fi in ret[0][1:]:
                ret[-1].append(str(count.get((tn, fi), 0)))

        return ret
```
