#include "luhn.h"

#include <ctype.h>
#include <stdio.h>

bool luhn(const char *num)
{
    char cleaned[50] = "";
    int len = 0;
    for (int i = 0; num[i] != '\0'; i++) {
        if (isspace(num[i])) continue;
        if (!isdigit(num[i])) return false;
        cleaned[len] = num[i];
        len++;
    }
    cleaned[len] = '\0'; 
    if (len <= 1) return false;

    int sum = 0;
    bool doubling = false;
    for (int i = len - 1; i >= 0; i--) {
        int digit = cleaned[i] - '0';
        sum += doubling ? double_digit(digit) : digit ;
        doubling = !doubling;
    }
    
    return sum % 10 == 0;
}

int double_digit(int digit)
{
    int doubled = digit * 2;
    return doubled > 9 ? doubled - 9 : doubled;
}