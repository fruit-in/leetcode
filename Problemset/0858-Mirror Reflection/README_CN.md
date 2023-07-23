# 858. 镜面反射
有一个特殊的正方形房间，每面墙上都有一面镜子。除西南角以外，每个角落都放有一个接受器，编号为 `0`， `1`，以及 `2`。

正方形房间的墙壁长度为 `p`，一束激光从西南角射出，首先会与东墙相遇，入射点到接收器 `0` 的距离为 `q` 。

返回光线最先遇到的接收器的编号（保证光线最终会遇到一个接收器）。

#### 示例:
<pre>
<strong>输入:</strong> p = 2, q = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 这条光线在第一次被反射回左边的墙时就遇到了接收器 2 。
<img src="https://aliyun-lc-upload.oss-cn-hangzhou.aliyuncs.com/aliyun-lc-upload/uploads/2018/06/22/reflection.png">
</pre>

#### 提示:
1. `1 <= p <= 1000`
2. `0 <= q <= p`

## 题解 (Ruby)

### 1. 数学
```Ruby
# @param {Integer} p
# @param {Integer} q
# @return {Integer}
def mirror_reflection(p, q)
    a, b = p, q

    while b != 0
        a, b = b, a % b
    end

    if p / a % 2 == 0
        return 2
    elsif q / a % 2 == 0
        return 0
    else
        return 1
    end
end
```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut a = p;
        let mut b = q;

        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }

        match (p / a % 2, q / a % 2) {
            (0, _) => 2,
            (1, 0) => 0,
            (_, _) => 1,
        }
    }
}
```
