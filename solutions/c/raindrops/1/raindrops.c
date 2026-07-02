#include "raindrops.h"
#include <string.h>
#include <stdio.h>
#include <stdbool.h>

void convert(char result[], int drops)
{
    bool div_3 = drops % 3 == 0;
    if (div_3) strcat(result, "Pling");
    
    bool div_5 = drops % 5 == 0;
    if (div_5) strcat(result, "Plang");
    
    bool div_7 = drops % 7 == 0;
    if (div_7) strcat(result, "Plong");
    
    if (!div_3 && !div_5 && !div_7) sprintf(result, "%d", drops);
}