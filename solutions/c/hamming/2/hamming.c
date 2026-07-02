#include "hamming.h"

int compute(const char *lhs, const char *rhs)
{
    int dist = 0;
    while (*lhs != '\0' && *rhs != '\0') {
        if (*lhs != *rhs) dist++;
        lhs++;
        rhs++;
    }
    if (*lhs != '\0' || *rhs != '\0') return -1;
    return dist;
}