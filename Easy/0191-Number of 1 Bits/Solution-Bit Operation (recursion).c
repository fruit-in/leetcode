int hammingWeight(uint32_t n){
    return n ? hammingWeight(n >> 1) + (n & 1) : 0;
}
