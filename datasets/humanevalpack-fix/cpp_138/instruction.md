# Context

```cpp
/*
Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
Example
is_equal_to_sum_even(4) == false
is_equal_to_sum_even(6) == false
is_equal_to_sum_even(8) == true
*/
#include<stdio.h>
using namespace std;
bool is_equal_to_sum_even(int n){
    if (n%2==0 and n>=8 and n <=8) return true;
    return false;
}
```

# Instruction

Fix bugs in is_equal_to_sum_even.

# Prompt

/*
Evaluate whether the given number n can be written as the sum of exactly 4 positive even numbers
Example
is_equal_to_sum_even(4) == false
is_equal_to_sum_even(6) == false
is_equal_to_sum_even(8) == true
*/
#include<stdio.h>
using namespace std;
bool is_equal_to_sum_even(int n){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
