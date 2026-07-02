#include "armstrong_numbers.h"

#include<math.h>
#include<stdio.h>

#define MAX_DIGITS 10

bool is_armstrong_number(int candidate)
{
    int digits[MAX_DIGITS] = {0};
    int n_digits = 0;
    int current = candidate;
    for(int i = 1; current != 0; i++) {
        digits[i] = current % 10;
        n_digits++;
        current /= 10;
    }
    
    if (n_digits == 0) return true;

    int sum = 0;
    for(int i = 0; i < MAX_DIGITS; i++) {
        sum += (int)pow(digits[i], n_digits);
        if (sum > candidate) {
            return false;
        }
    }
    return sum == candidate;
}