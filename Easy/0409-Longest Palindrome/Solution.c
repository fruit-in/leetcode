int longestPalindrome(char * s){
    int count[52] = {};
    int result = 0;
    while(*s != '\0'){
        if(*s <= 'Z')
            count[*s - 'A']++;
        else if(*s <= 'z')
            count[*s - 'a' + 26]++;
        s++;
    }
    for(int i = 0; i < 52; i++){
        if(count[i] % 2 == 0 || result % 2 == 0)
            result += count[i];
        else
            result += count[i] - 1;
    }
    return result;
}
