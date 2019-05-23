int hammingWeight(uint32_t n){
    int ones = 0;
    do{
        ones += n & 1;
    }while(n >>= 1);
    return ones;
}
