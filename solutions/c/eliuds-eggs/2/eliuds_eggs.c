#include "eliuds_eggs.h"

int egg_count(int decimal)
{
    int eggs = 0;
    int current = decimal;
    while (current > 0) {
        eggs += current % 2;
        current /= 2;
    }
    return eggs;
}