# 1710. 卡车上的最大单元数
请你将一些箱子装在 **一辆卡车** 上。给你一个二维数组 `boxTypes` ，其中 <code>boxTypes[i] = [numberOfBoxes<sub>i</sub>, numberOfUnitsPerBox<sub>i</sub>]</code> ：
* <code>numberOfBoxes<sub>i</sub></code> 是类型 `i` 的箱子的数量。
* <code>numberOfUnitsPerBox<sub>i</sub></code> 是类型 `i` 每个箱子可以装载的单元数量。

整数 `truckSize` 表示卡车上可以装载 **箱子** 的 **最大数量** 。只要箱子数量不超过 `truckSize` ，你就可以选择任意箱子装到卡车上。

返回卡车可以装载 **单元** 的 **最大** 总数。

#### 示例 1:
<pre>
<strong>输入:</strong> boxTypes = [[1,3],[2,2],[3,1]], truckSize = 4
<strong>输出:</strong> 8
<strong>解释:</strong> 箱子的情况如下：
- 1 个第一类的箱子，里面含 3 个单元。
- 2 个第二类的箱子，每个里面含 2 个单元。
- 3 个第三类的箱子，每个里面含 1 个单元。
可以选择第一类和第二类的所有箱子，以及第三类的一个箱子。
单元总数 = (1 * 3) + (2 * 2) + (1 * 1) = 8
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
<strong>输出:</strong> 91
</pre>

#### 提示:
* `1 <= boxTypes.length <= 1000`
* <code>1 <= numberOfBoxes<sub>i</sub>, numberOfUnitsPerBox<sub>i</sub> <= 1000</code>
* <code>1 <= truckSize <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|b| b[1]);
        let mut ret = 0;

        while let Some(b) = box_types.pop() {
            ret += b[0].min(truck_size) * b[1];
            truck_size -= b[0];
            if truck_size <= 0 {
                break;
            }
        }

        ret
    }
}
```
