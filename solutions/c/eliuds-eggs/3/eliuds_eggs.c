#include "eliuds_eggs.h"

int egg_count(int decimal)
{
    int eggs = 0;
    while (decimal > 0) {
        eggs += decimal & 1;
        decimal >>= 1;
    }
    return eggs;
}