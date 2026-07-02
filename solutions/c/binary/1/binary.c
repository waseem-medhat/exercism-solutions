#include "binary.h"
#include <math.h>
#include <stdio.h>

int convert(const char *input)
{
    int len = 0;
    for (int i = 0; input[i] != '\0'; i++) {
        len++;
    }
    
    
    int num = 0;
    for (int i = len - 1; i >= 0; i--) {
        int digit = input[i] - 48;
        if (digit != 0 && digit != 1) return -1;
        int power = len - 1 - i;
        num += digit * (int)(pow(2, power));
    }

    return num;
}