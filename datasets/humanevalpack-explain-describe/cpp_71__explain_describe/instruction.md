# Context

You are given a reference implementation (canonical solution) to explain.

```cpp
/*
Given the lengths of the three sides of a triangle. Return the area of
the triangle rounded to 2 decimal points if the three sides form a valid triangle. 
Otherwise return -1
Three sides make a valid triangle when the sum of any two sides is greater 
than the third side.
Example:
triangle_area(3, 4, 5) == 6.00
triangle_area(1, 2, 10) == -1
*/
#include<stdio.h>
#include<math.h>
using namespace std;
float triangle_area(float a,float b,float c){
    if (a+b<=c or a+c<=b or b+c<=a) return -1;
    float h=(a+b+c)/2;
    float area;
    area=pow(h*(h-a)*(h-b)*(h-c),0.5);
    return area;
}
```

# Instruction

Explain the algorithm in natural language:
- Describe the core idea and step-by-step approach
- Specify inputs/outputs and edge cases
- Provide time/space complexity

Constraints:
- Output must be plain text only
- Do NOT output any code
- Write your explanation to `solution/explanation.txt`

# Prompt

/*
Given the lengths of the three sides of a triangle. Return the area of
the triangle rounded to 2 decimal points if the three sides form a valid triangle. 
Otherwise return -1
Three sides make a valid triangle when the sum of any two sides is greater 
than the third side.
Example:
triangle_area(3, 4, 5) == 6.00
triangle_area(1, 2, 10) == -1
*/
#include<stdio.h>
#include<math.h>
using namespace std;
float triangle_area(float a,float b,float c){


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
