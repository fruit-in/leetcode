# 1024. Video Stitching
You are given a series of video clips from a sporting event that lasted `time` seconds. These video clips can be overlapping with each other and have varying lengths.

Each video clip is described by an array `clips` where `clips[i] = [starti, endi]` indicates that the ith clip started at `starti` and ended at `endi`.

We can cut these clips into segments freely.

* For example, a clip `[0, 7]` can be cut into segments `[0, 1] + [1, 3] + [3, 7]`.

Return *the minimum number of clips needed so that we can cut the clips into segments that cover the entire sporting event* `[0, time]`. If the task is impossible, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], time = 10
<strong>Output:</strong> 3
<strong>Explanation:</strong> We take the clips [0,2], [8,10], [1,9]; a total of 3 clips.
Then, we can reconstruct the sporting event as follows:
We cut [1,9] into segments [1,2] + [2,8] + [8,9].
Now we have segments [0,2] + [2,8] + [8,10] which cover the sporting event [0, 10].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> clips = [[0,1],[1,2]], time = 5
<strong>Output:</strong> -1
<strong>Explanation:</strong> We cannot cover [0,5] with only [0,1] and [1,2].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]], time = 9
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can take clips [0,4], [4,7], and [6,9].
</pre>

#### Constraints:
* `1 <= clips.length <= 100`
* `0 <= starti <= endi <= 100`
* `1 <= time <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let mut clips = clips;
        let mut start = 0;
        let mut ret = 0;

        clips.sort_unstable();

        for i in 1..clips.len() {
            if clips[i][1] < clips[i - 1][1] {
                clips[i][1] = clips[i - 1][1];
            }
        }

        while start < t {
            match clips.binary_search(&vec![start, i32::MAX]) {
                Err(i) if i > 0 => {
                    if start >= clips[i - 1][1] {
                        return -1;
                    }

                    start = clips[i - 1][1];
                    ret += 1;
                }
                _ => return -1,
            }
        }

        ret
    }
}
```
