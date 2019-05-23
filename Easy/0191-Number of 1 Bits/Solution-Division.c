int hammingWeight(uint32_t n){
    int ones = 0;
    uint32_t twos;
    for(uint32_t twos = (uint32_t)pow(2, 31); n != 0; twos /= 2){
        ones += n / twos;
        n %= twos;
    }
    return ones;
}
