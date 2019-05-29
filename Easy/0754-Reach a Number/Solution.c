int reachNumber(int target){
    int position = 0, step = 0;
    if(target < 0)
        target = -target;
    while(position < target){
        step++;
        position += step;
    }
    if((position - target) % 2 == 0)
        return step;
    if(step % 2 == 0)
        return step + 1;
    return step + 2;
}
