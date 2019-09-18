# 11. Container With Most Water

Given *n* non-negative integers *a<sub>1</sub>*, *a<sub>2</sub>*, ..., *a<sub>n</sub>* , where each represents a point at coordinate (*i*, *a<sub>i</sub>*). *n* vertical lines are drawn such that the two endpoints of line *i* is at (*i*, *a<sub>i</sub>*) and (*i*, 0). Find two lines, which together with x-axis forms a container, such that the container contains the most water.

**Note:** You may not slant the container and *n* is at least 2.

![img](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg)

The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.

**Example:**

```
Input: [1,8,6,2,5,4,8,3,7]
Output: 49
```

## Solutions (C)

### 1.Two Pointers

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
