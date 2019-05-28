int binaryGap(int N){
    int max = 0;
    int gap = 0;
    while((N & 1) == 0)
        N >>= 1;
    while(N != 0){
        N >>= 1;
        gap++;
        if(N & 1 == 1){
            max = max > gap ? max : gap;
            gap = 0;
        }
    }
    return max;
}
