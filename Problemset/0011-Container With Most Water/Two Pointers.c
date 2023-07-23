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
