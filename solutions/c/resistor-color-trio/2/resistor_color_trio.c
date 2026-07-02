#include "resistor_color_trio.h"
#include <math.h>
#include <stdio.h>
#include <limits.h>

resistor_value_t color_code(resistor_band_t bands[])
{    
    long multiplier = (int)pow(10, bands[2]);
    long val = (10 * bands[0] + bands[1]) * multiplier;

    if (val < KILOOHMS) {
        return (resistor_value_t){
            .value = val,
            .unit = OHMS
        };
    }
    if (val < MEGAOHMS) {
        return (resistor_value_t){
            .value = val / KILOOHMS,
            .unit = KILOOHMS
        };
    }
    if (val < GIGAOHMS) {
        return (resistor_value_t){
            .value = val / MEGAOHMS,
            .unit = MEGAOHMS
        };
    }
    return (resistor_value_t){
        .value = val / GIGAOHMS,
        .unit = GIGAOHMS
    };

}