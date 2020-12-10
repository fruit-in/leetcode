# 957. Prison Cells After N Days
There are 8 prison cells in a row, and each cell is either occupied or vacant.

Each day, whether the cell is occupied or vacant changes according to the following rules:
* If a cell has two adjacent neighbors that are both occupied or both vacant, then the cell becomes occupied.
* Otherwise, it becomes vacant.

(Note that because the prison is a row, the first and the last cells in the row can't have two adjacent neighbors.)

We describe the current state of the prison in the following way: `cells[i] == 1` if the `i`-th cell is occupied, else `cells[i] == 0`.

Given the initial state of the prison, return the state of the prison after `N` days (and `N` such changes described above.)

#### Example 1:
<pre>
<strong>Input:</strong> cells = [0,1,0,1,1,0,0,1], N = 7
<strong>Output:</strong> [0,0,1,1,0,0,0,0]
<strong>Explanation:</strong>
The following table summarizes the state of the prison on each day:
Day 0: [0, 1, 0, 1, 1, 0, 0, 1]
Day 1: [0, 1, 1, 0, 0, 0, 0, 0]
Day 2: [0, 0, 0, 0, 1, 1, 1, 0]
Day 3: [0, 1, 1, 0, 0, 1, 0, 0]
Day 4: [0, 0, 0, 0, 0, 1, 0, 0]
Day 5: [0, 1, 1, 1, 0, 1, 0, 0]
Day 6: [0, 0, 1, 0, 1, 1, 0, 0]
Day 7: [0, 0, 1, 1, 0, 0, 0, 0]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cells = [1,0,0,1,0,0,1,0], N = 1000000000
<strong>Output:</strong> [0,0,1,1,1,1,1,0]
</pre>

#### Note:
1. `cells.length == 8`
2. `cells[i]` is in `{0, 1}`
3. `1 <= N <= 10^9`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        Self::prison_after_one_day(&mut cells);

        let origin = cells.clone();
        let mut period = n;

        for i in 1..n {
            Self::prison_after_one_day(&mut cells);

            if cells[1..7] == origin[1..7] {
                period = i;
                break;
            }
        }

        for _ in 0..((n - 1 - period) % period) {
            Self::prison_after_one_day(&mut cells);
        }

        cells
    }

    pub fn prison_after_one_day(cells: &mut Vec<i32>) {
        let next_day = (1..7)
            .map(|i| 1 - (cells[i - 1] ^ cells[i + 1]))
            .collect::<Vec<_>>();

        cells[0] = 0;
        cells[7] = 0;
        for i in 1..7 {
            cells[i] = next_day[i - 1];
        }
    }
}
```
