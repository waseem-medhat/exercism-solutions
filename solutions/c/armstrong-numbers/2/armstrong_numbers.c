#include "armstrong_numbers.h"

#include<math.h>

#define MAX_DIGITS 10

bool is_armstrong_number(int candidate)
{
    int n_digits = 0;
    int current = candidate;
    while (current > 0) {
        n_digits++;
        current /= 10;
    }
    
    int sum = 0;
    current = candidate;
    while(current > 0) {
        int digit = current % 10;
        sum += (int)pow(digit, n_digits);
        if (sum > candidate) {
            return false;
        }
        current /= 10;
    }
    return sum == candidate;
}