int numJewelsInStones(char * J, char * S){
    int jewels = 0;
    while(*J != '\0') {
        char i = 0;
        while(S[i] != '\0') {
            if(S[i] == *J)
                jewels++;
            i++;
        }
        J++;
    }
    return jewels;
}
