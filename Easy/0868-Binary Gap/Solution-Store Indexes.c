int binaryGap(int N){
    int max = 0;
    int indexes[32] = {};
    for(int i = 0, j = 0; i < 32; i++)
        if(((N >> i) & 1) == 1){
            indexes[j] = i;
            j++;
        }
    for(int i = 1; indexes[i] > indexes[i - 1]; i++)
        max = max > indexes[i] - indexes[i - 1] ? max : indexes[i] - indexes[i - 1];
    return max;
}
