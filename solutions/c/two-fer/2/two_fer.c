#include "two_fer.h"

#include <string.h>
#include <stdio.h>

char *PREFIX = "One for ";
char *SUFFIX = ", one for me.";

void two_fer(char *buffer, const char *name)
{
    sprintf(buffer, "One for %s, one for me.", name ? name : "you");
}