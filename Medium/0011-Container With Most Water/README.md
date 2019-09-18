# 11. Container With Most Water

Given *n* non-negative integers *a<sub>1</sub>*, *a<sub>2</sub>*, ..., *a<sub>n</sub>* , where each represents a point at coordinate (*i*, *a<sub>i</sub>*). *n* vertical lines are drawn such that the two endpoints of line *i* is at (*i*, *a<sub>i</sub>*) and (*i*, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.

**Note:** You may not slant the container and *n* is at least 2.

![img](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg)

The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.

**Example:**

<pre>
<strong>Input:</strong> [1,8,6,2,5,4,8,3,7]
<strong>Output:</strong> 49
</pre>

## Solutions (Rust)

### 1. Brute Force
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

### 2. Two Pointers
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

## Solutions (C)

### 1. Two Pointers

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
