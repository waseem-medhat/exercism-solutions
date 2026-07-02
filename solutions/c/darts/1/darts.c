#include "darts.h"
#include <math.h>
#include <stdint.h>

uint8_t score(coordinate_t landing_position)
{
    float x = landing_position[0];
    float y = landing_position[1];
    float distance = sqrt(x * x + y * y);
    
    if (distance > 10) return 0;
    if (distance > 5) return 1;
    if (distance > 1) return 5;
    return 10;
}