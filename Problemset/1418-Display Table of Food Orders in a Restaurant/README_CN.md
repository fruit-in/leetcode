# 1418. 点菜展示表
给你一个数组 `orders`，表示客户在餐厅中完成的订单，确切地说， <code>orders[i]=[customerName<sub>i</sub>,tableNumber<sub>i</sub>,foodItem<sub>i</sub>]</code> ，其中 <code>customerName<sub>i</sub></code> 是客户的姓名，<code>tableNumber<sub>i</sub></code> 是客户所在餐桌的桌号，而 <code>foodItem<sub>i</sub></code> 是客户点的餐品名称。

请你返回该餐厅的 **点菜展示表** 。在这张表中，表中第一行为标题，其第一列为餐桌桌号 “Table” ，后面每一列都是按字母顺序排列的餐品名称。接下来每一行中的项则表示每张餐桌订购的相应餐品数量，第一列应当填对应的桌号，后面依次填写下单的餐品数量。

注意：客户姓名不是点菜展示表的一部分。此外，表中的数据行应该按餐桌桌号升序排列。

#### 示例 1:
<pre>
<strong>输入:</strong> orders = [["David","3","Ceviche"],["Corina","10","Beef Burrito"],["David","3","Fried Chicken"],["Carla","5","Water"],["Carla","5","Ceviche"],["Rous","3","Ceviche"]]
<strong>输出:</strong> [["Table","Beef Burrito","Ceviche","Fried Chicken","Water"],["3","0","2","1","0"],["5","0","1","0","1"],["10","1","0","0","0"]]
<strong>解释:</strong>
点菜展示表如下所示：
Table,Beef Burrito,Ceviche,Fried Chicken,Water
3    ,0           ,2      ,1            ,0
5    ,0           ,1      ,0            ,1
10   ,1           ,0      ,0            ,0
对于餐桌 3：David 点了 "Ceviche" 和 "Fried Chicken"，而 Rous 点了 "Ceviche"
而餐桌 5：Carla 点了 "Water" 和 "Ceviche"
餐桌 10：Corina 点了 "Beef Burrito"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> orders = [["James","12","Fried Chicken"],["Ratesh","12","Fried Chicken"],["Amadeus","12","Fried Chicken"],["Adam","1","Canadian Waffles"],["Brianna","1","Canadian Waffles"]]
<strong>输出:</strong> [["Table","Canadian Waffles","Fried Chicken"],["1","2","0"],["12","0","3"]]
<strong>解释:</strong>
对于餐桌 1：Adam 和 Brianna 都点了 "Canadian Waffles"
而餐桌 12：James, Ratesh 和 Amadeus 都点了 "Fried Chicken"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> orders = [["Laura","2","Bean Burrito"],["Jhon","2","Beef Burrito"],["Melissa","2","Soda"]]
<strong>输出:</strong> [["Table","Bean Burrito","Beef Burrito","Soda"],["2","1","1","1"]]
</pre>

#### 提示:
* `1 <= orders.length <= 5 * 10^4`
* `orders[i].length == 3`
* <code>1 <= customerName<sub>i</sub>.length, foodItem<sub>i</sub>.length <= 20</code>
* <code>customerName<sub>i</sub></code> 和 <code>foodItem<sub>i</sub></code> 由大小写英文字母及空格字符 `' '` 组成。
* <code>tableNumber<sub>i</sub></code> 是 `1` 到 `500` 范围内的整数。

## 题解 (Python)

### 1. 题解
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
