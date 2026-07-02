#include "gigasecond.h"
#include<stdio.h>
#include<time.h>

#define GIGASEC 1000000000

void gigasecond(time_t input, char *output, size_t size)
{
    time_t after = input + GIGASEC;
    strftime(output, size, "%Y-%m-%d %H:%M:%S", gmtime(&after));
}