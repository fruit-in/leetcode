bool judgeCircle(char * moves){
    int x = 0;
    int y = 0;
    for(char* ch = moves; *ch != NULL; ch++){
        switch(*ch){
            case 'R': x++; break;
            case 'L': x--; break;
            case 'U': y++; break;
            case 'D': y--; break;
        }
    }
    return x == 0 && y == 0;
}
