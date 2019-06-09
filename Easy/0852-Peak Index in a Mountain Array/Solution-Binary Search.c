int peakIndexInMountainArray(int* A, int ASize){
    int head = 0, tail = ASize - 1;
    while(true){
        int mid = (head + tail) / 2;
        if(A[mid - 1] < A[mid] && A[mid] > A[mid + 1])
            return mid;
        else if(A[mid] < A[mid + 1])
            head = mid;
        else if(A[mid] > A[mid + 1])
            tail = mid;
    }
}
