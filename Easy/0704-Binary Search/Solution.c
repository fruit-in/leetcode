int search(int* nums, int numsSize, int target){
    int head = 0, tail = numsSize - 1;
    while(head <= tail){
        if(target == nums[(head + tail) / 2])
            return (head + tail) / 2;
        else if(target > nums[(head + tail) / 2])
            head = (head + tail) / 2 + 1;
        else if(target < nums[(head + tail) / 2])
            tail = (head + tail) / 2 - 1;
    }
    return -1;
}
