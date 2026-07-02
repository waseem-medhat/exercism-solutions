#include "binary_search.h"

#include <stdio.h>

const int *binary_search(int value, const int *arr, size_t length) {
    int lo = 0;
    int hi = length - 1;
    int md = (hi + lo) / 2;
    while (hi >= lo) {
        if (arr[md] == value) return &arr[md];
        if (arr[md] > value) {
            hi = md - 1;
        } else {            
            lo = md + 1;
        }
        md = (hi + lo) / 2;
    }
    return NULL;
}