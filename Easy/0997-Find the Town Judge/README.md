# 997. Find the Town Judge
In a town, there are ```N``` people labelled from ```1``` to ```N```.  There is a rumor that one of these people is secretly the town judge.

If the town judge exists, then:
1. The town judge trusts nobody.
2. Everybody (except for the town judge) trusts the town judge.
3. There is exactly one person that satisfies properties 1 and 2.

You are given ```trust```, an array of pairs ```trust[i] = [a, b]``` representing that the person labelled ```a``` trusts the person labelled ```b```.

If the town judge exists and can be identified, return the label of the town judge.  Otherwise, return ```-1```.

#### Example 1:
<pre>
<strong>Input:</strong> N = 2, trust = [[1,2]]
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> N = 3, trust = [[1,3],[2,3]]
<strong>Output:</strong> 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> N = 3, trust = [[1,3],[2,3],[3,1]]
<strong>Output:</strong> -1
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> N = 3, trust = [[1,2],[2,3]]
<strong>Output:</strong> -1
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> N = 4, trust = [[1,3],[1,4],[2,3],[2,4],[4,3]]
<strong>Output:</strong> 3
</pre>

#### Note:
1. ```1 <= N <= 1000```
2. ```trust.length <= 10000```
3. ```trust[i]``` are all different
4. ```trust[i][0] != trust[i][1]```
5. ```1 <= trust[i][0], trust[i][1] <= N```

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut people = vec![0; n as usize];

        for t in trust {
            people[t[0] as usize - 1] -= 1;
            people[t[1] as usize - 1] += 1;
        }

        for i in 0..n {
            if people[i as usize] == n - 1 {
                return i + 1;
            }
        }

        -1
    }
}
```
