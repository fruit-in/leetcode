# 1606. Find Servers That Handled Most Number of Requests
You have `k` servers numbered from `0` to `k-1` that are being used to handle multiple requests simultaneously. Each server has infinite computational capacity but **cannot handle more than one request at a time**. The requests are assigned to servers according to a specific algorithm:

* The <code>i<sup>th</sup></code> (0-indexed) request arrives.
* If all servers are busy, the request is dropped (not handled at all).
* If the <code>(i % k)<sup>th</sup></code> server is available, assign the request to that server.
* Otherwise, assign the request to the next available server (wrapping around the list of servers and starting from 0 if necessary). For example, if the <code>i<sup>th</sup></code> server is busy, try to assign the request to the <code>(i+1)<sup>th</sup></code> server, then the <code>(i+2)<sup>th</sup></code> server, and so on.

You are given a **strictly increasing** array `arrival` of positive integers, where `arrival[i]` represents the arrival time of the <code>i<sup>th</sup></code> request, and another array `load`, where `load[i]` represents the load of the <code>i<sup>th</sup></code> request (the time it takes to complete). Your goal is to find the **busiest server(s)**. A server is considered **busiest** if it handled the most number of requests successfully among all the servers.

Return *a list containing the IDs (0-indexed) of the **busiest server(s)***. You may return the IDs in any order.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/08/load-1.png)
<pre>
<strong>Input:</strong> k = 3, arrival = [1,2,3,4,5], load = [5,2,3,3,3]
<strong>Output:</strong> [1]
<strong>Explanation:</strong>
All of the servers start out available.
The first 3 requests are handled by the first 3 servers in order.
Request 3 comes in. Server 0 is busy, so it's assigned to the next available server, which is 1.
Request 4 comes in. It cannot be handled since all servers are busy, so it is dropped.
Servers 0 and 2 handled one request each, while server 1 handled two requests. Hence server 1 is the busiest server.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 3, arrival = [1,2,3,4], load = [1,2,1,2]
<strong>Output:</strong> [0]
<strong>Explanation:</strong>
The first 3 requests are handled by first 3 servers.
Request 3 comes in. It is handled by server 0 since the server is available.
Server 0 handled two requests, while servers 1 and 2 handled one request each. Hence server 0 is the busiest server.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 3, arrival = [1,2,3], load = [10,12,11]
<strong>Output:</strong> [0,1,2]
<strong>Explanation:</strong> Each server handles a single request, so they are all considered the busiest.
</pre>

#### Constraints:
* <code>1 <= k <= 10<sup>5</sup></code>
* <code>1 <= arrival.length, load.length <= 10<sup>5</sup></code>
* `arrival.length == load.length`
* <code>1 <= arrival[i], load[i] <= 10<sup>9</sup></code>
* `arrival` is **strictly increasing**.

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut free = (0..k).map(|i| Reverse(i)).collect::<BinaryHeap<_>>();
        let mut busy = BinaryHeap::new();
        let mut count = vec![0; load.len()];

        for i in 0..load.len() {
            while let Some(&(Reverse(t), j)) = busy.peek() {
                if t > arrival[i] {
                    break;
                } else if i % k > j {
                    free.push(Reverse(j + i / k * k + k));
                } else {
                    free.push(Reverse(j + i / k * k));
                }
                busy.pop();
            }

            if let Some(Reverse(j)) = free.pop() {
                busy.push((Reverse(arrival[i] + load[i]), j % k));
                count[j % k] += 1;
            }
        }

        let max_count = *count.iter().max().unwrap();

        (0..k as i32)
            .filter(|&i| *count.get(i as usize).unwrap_or(&0) == max_count)
            .collect()
    }
}
```
