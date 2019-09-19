int reverse(int x)
{
    int Int_Max = 0x7fffffff;
    int Int_Min = 0x80000000;
    long sum = 0;
    
    for( ; x; x /= 10 )
    {
       sum = ( sum *= 10 ) + x % 10;
    }
    
    if( sum > Int_Max || sum < Int_Min )
        return 0;
    return sum;
}

