# 1710. Maximum Units on a Truck
You are assigned to put some amount of boxes onto **one truck**. You are given a 2D array `boxTypes`, where <code>boxTypes[i] = [numberOfBoxes<sub>i</sub>, numberOfUnitsPerBox<sub>i</sub>]</code>:
* <code>numberOfBoxes<sub>i</sub></code> is the number of boxes of type `i`.
* <code>numberOfUnitsPerBox<sub>i</sub></code> is the number of units in each box of the type `i`.

You are also given an integer `truckSize`, which is the **maximum** number of **boxes** that can be put on the truck. You can choose any boxes to put on the truck as long as the number of boxes does not exceed `truckSize`.

Return *the **maximum** total number of **units** that can be put on the truck*.

#### Example 1:
<pre>
<strong>Input:</strong> boxTypes = [[1,3],[2,2],[3,1]], truckSize = 4
<strong>Output:</strong> 8
<strong>Explanation:</strong> There are:
- 1 box of the first type that contains 3 units.
- 2 boxes of the second type that contain 2 units each.
- 3 boxes of the third type that contain 1 unit each.
You can take all the boxes of the first and second types, and one box of the third type.
The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> boxTypes = [[5,10],[2,5],[4,7],[3,9]], truckSize = 10
<strong>Output:</strong> 91
</pre>

#### Constraints:
* `1 <= boxTypes.length <= 1000`
* <code>1 <= numberOfBoxes<sub>i</sub>, numberOfUnitsPerBox<sub>i</sub> <= 1000</code>
* <code>1 <= truckSize <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Sort
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
