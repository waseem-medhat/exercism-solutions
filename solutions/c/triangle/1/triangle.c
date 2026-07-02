#include "triangle.h"

#include <stdbool.h>

bool is_equilateral(triangle_t triangle)
{
    return is_valid_triangle(triangle) &&
        triangle.a == triangle.b &&
        triangle.b == triangle.c;
}

bool is_isosceles(triangle_t triangle)
{
    return is_valid_triangle(triangle) &&
        (triangle.a == triangle.b ||
         triangle.b == triangle.c ||
         triangle.c == triangle.a);
}

bool is_scalene(triangle_t triangle)
{
    return is_valid_triangle(triangle) &&
        triangle.a != triangle.b &&
        triangle.b != triangle.c &&
        triangle.c != triangle.a;
}

bool is_valid_triangle(triangle_t triangle)
{
    double a = triangle.a;
    double b = triangle.b;
    double c = triangle.c;
    return a && b && c &&
        a + b >= c &&
        b + c >= a &&
        a + c >= b;
}