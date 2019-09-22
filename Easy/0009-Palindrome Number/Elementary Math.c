  bool isPalindrome(int x){
    int origin = x;
    long sum = 0;
    while(x > 0)
    {
        sum = sum *10 + x % 10;
        x /= 10;
    }
    if(sum == origin)
    {
        return true;
    }
    return false;
}

