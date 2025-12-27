# Context

You are given a reference implementation (canonical solution) to explain.

```java
import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given the lengths of the three sides of a triangle. Return true if the three
    sides form a right-angled triangle, false otherwise.
    A right-angled triangle is a triangle in which one angle is right angle or
    90 degree.
    Example:
    rightAngleTriangle(3, 4, 5) == true
    rightAngleTriangle(1, 2, 3) == false
     */
    public boolean rightAngleTriangle(int a, int b, int c) {
        return a * a == b * b + c * c || b * b == a * a + c * c || c * c == a * a + b * b;
    }
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

import java.util.*;
import java.lang.*;

class Solution {
    /**
    Given the lengths of the three sides of a triangle. Return true if the three
    sides form a right-angled triangle, false otherwise.
    A right-angled triangle is a triangle in which one angle is right angle or
    90 degree.
    Example:
    rightAngleTriangle(3, 4, 5) == true
    rightAngleTriangle(1, 2, 3) == false
     */
    public boolean rightAngleTriangle(int a, int b, int c) {


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
