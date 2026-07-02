#include "two_fer.h"

#include <string.h>
#include <stdio.h>

char *PREFIX = "One for ";
char *SUFFIX = ", one for me.";

void two_fer(char *buffer, const char *name)
{
    memset(buffer, '\0', strlen(buffer));
    strcat(buffer, PREFIX);
    if (name != NULL) {
        strcat(buffer, name);
    } else {
        strcat(buffer, "you");
    }
    strcat(buffer, SUFFIX);
}