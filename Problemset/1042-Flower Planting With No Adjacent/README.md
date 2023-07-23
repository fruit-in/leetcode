# 1042. Flower Planting With No Adjacent
You have ```N``` gardens, labelled ```1``` to ```N```.  In each garden, you want to plant one of 4 types of flowers.

```paths[i] = [x, y]``` describes the existence of a bidirectional path from garden ```x``` to garden ```y```.

Also, there is no garden that has more than 3 paths coming into or leaving it.

Your task is to choose a flower type for each garden such that, for any two gardens connected by a path, they have different types of flowers.

Return **any** such a choice as an array ```answer```, where ```answer[i]``` is the type of flower planted in the ```(i+1)```-th garden.  The flower types are denoted 1, 2, 3, or 4.  It is guaranteed an answer exists.

#### Example 1:
<pre>
<strong>Input:</strong> N = 3, paths = [[1,2],[2,3],[3,1]]
<strong>Output:</strong> [1,2,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> N = 4, paths = [[1,2],[3,4]]
<strong>Output:</strong> [1,2,1,2]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> N = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
<strong>Output:</strong> [1,2,3,4]
</pre>

#### Note:
* ```1 <= N <= 10000```
* ```0 <= paths.size <= 20000```
* No garden has 4 or more paths coming into or leaving it.
* It is guaranteed an answer exists.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut answer = vec![0; n as usize];
        let mut graph = vec![Vec::new(); n as usize];

        for path in paths {
            graph[path[0] as usize - 1].push(path[1] as usize - 1);
            graph[path[1] as usize - 1].push(path[0] as usize - 1);
        }

        for i in 0..answer.len() {
            let mut choice = 0;
            for &neighbor in &graph[i] {
                if neighbor < i {
                    choice |= 1 << (answer[neighbor] - 1);
                }
            }

            match choice {
                7 => answer[i] = 4,
                3|11 => answer[i] = 3,
                1|5|9|13 => answer[i] = 2,
                _ => answer[i] = 1,
            };
        }

        answer
    }
}
```
