int smallestRangeI(int* A, int ASize, int K){
    int i, min, max;
    for(i = 1, min = A[0], max = A[0]; i < ASize; i++){
        if(A[i] < min)
            min = A[i];
        if(A[i] > max)
            max = A[i];
    }
    return (max - min < 2 * K) ? 0 : (max - min - 2 * K);
}
