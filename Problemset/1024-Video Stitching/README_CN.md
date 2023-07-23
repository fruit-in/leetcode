# 1024. 视频拼接
你将会获得一系列视频片段，这些片段来自于一项持续时长为 `time` 秒的体育赛事。这些片段可能有所重叠，也可能长度不一。

使用数组 `clips` 描述所有的视频片段，其中 `clips[i] = [starti, endi]` 表示：某个视频片段开始于 `starti` 并于 `endi` 结束。

甚至可以对这些片段自由地再剪辑：
* 例如，片段 `[0, 7]` 可以剪切成 `[0, 1] + [1, 3] + [3, 7]` 三部分。

我们需要将这些片段进行再剪辑，并将剪辑后的内容拼接成覆盖整个运动过程的片段（`[0, time]`）。返回所需片段的最小数目，如果无法完成该任务，则返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], time = 10
<strong>输出:</strong> 3
<strong>解释:</strong> 选中 [0,2], [8,10], [1,9] 这三个片段。
然后，按下面的方案重制比赛片段：
将 [1,9] 再剪辑为 [1,2] + [2,8] + [8,9] 。
现在手上的片段为 [0,2] + [2,8] + [8,10]，而这些覆盖了整场比赛 [0, 10]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> clips = [[0,1],[1,2]], time = 5
<strong>输出:</strong> -1
<strong>解释:</strong> 无法只用 [0,1] 和 [1,2] 覆盖 [0,5] 的整个过程。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]], time = 9
<strong>输出:</strong> 3
<strong>解释:</strong> 选取片段 [0,4], [4,7] 和 [6,9] 。
</pre>

#### 提示:
* `1 <= clips.length <= 100`
* `0 <= starti <= endi <= 100`
* `1 <= time <= 100`

## 题解 (Rust)

### 1. 题解
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
