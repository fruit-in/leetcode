# 11. 盛最多水的容器
给定 *n* 个非负整数 *a*<sub>1</sub>，*a*<sub>2</sub>，...，*a*<sub>n</sub>，每个数代表坐标中的一个点 (*i*, *a<sub>i</sub>*) 。在坐标内画 *n* 条垂直线，垂直线 *i* 的两个端点分别为 (*i*, *a<sub>i</sub>*) 和 (*i*, 0)。找出其中的两条线，使得它们与 *x* 轴共同构成的容器可以容纳最多的水。

**说明:** 你不能倾斜容器，且 *n* 的值至少为 2。

![](https://aliyun-lc-upload.oss-cn-hangzhou.aliyuncs.com/aliyun-lc-upload/uploads/2018/07/25/question_11.jpg)

图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。

**示例:**

<pre>
<strong>Input:</strong> [1,8,6,2,5,4,8,3,7]
<strong>Output:</strong> 49
</pre>

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut most = 0;
        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                most = most.max((j - i) as i32 * height[i].min(height[j]));
            }
        }
        most
    }
}
```

### 2. 双指针
```Rust
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut most = 0;
        while l < r {
            most = most.max((r - l) as i32 * height[l].min(height[r]));
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        most
    }
}
```

## 题解 (C)

### 1. 双指针

```c
int maxArea(int* height, int heightSize){
    int *right,*left; 
    int volume = 0; 
    left = height;
    right = height + heightSize -1;
    
    for (heightSize--; heightSize != 0; heightSize--)
    {
        if (*right > *left)
        {
            volume = ((*left) * heightSize > volume) ? *left * heightSize : volume;
            left++;
        }
        else
        {
            volume = ((*right) * heightSize > volume) ? *right * heightSize : volume;
            right--;
        }
    }
    return volume;
}
```
