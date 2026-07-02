#include "darts.h"
#include <math.h>
#include <stdint.h>

uint8_t score(coordinate_t landing_position)
{
    float distance = hypotf(landing_position.x, landing_position.y);
    
    if (distance > 10) return 0;
    if (distance > 5) return 1;
    if (distance > 1) return 5;
    return 10;
}