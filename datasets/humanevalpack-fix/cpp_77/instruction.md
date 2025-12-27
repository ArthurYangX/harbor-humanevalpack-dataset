# Context

```cpp
/*
Write a function that takes an integer a and returns true 
if this ingeger is a cube of some integer number.
Note: you may assume the input is always valid.
Examples:
iscube(1) ==> true
iscube(2) ==> false
iscube(-1) ==> true
iscube(64) ==> true
iscube(0) ==> true
iscube(180) ==> false
*/
#include<stdio.h>
#include<math.h>
using namespace std;
bool iscube(int a){
    for (int i=0;i*i*i<=abs(a);i++)
        if (i*i==abs(a)) return true;
    return false;
}
```

# Instruction

Fix bugs in iscube.

# Prompt

/*
Write a function that takes an integer a and returns true 
if this ingeger is a cube of some integer number.
Note: you may assume the input is always valid.
Examples:
iscube(1) ==> true
iscube(2) ==> false
iscube(-1) ==> true
iscube(64) ==> true
iscube(0) ==> true
iscube(180) ==> false
*/
#include<stdio.h>
#include<math.h>
using namespace std;
bool iscube(int a){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
