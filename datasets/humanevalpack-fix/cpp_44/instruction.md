# Context

```cpp
/*
Change numerical base of input number x to base.
return string representation after the conversion.
base numbers are less than 10.
>>> change_base(8, 3)
"22"
>>> change_base(8, 2)
"1000"
>>> change_base(7, 2)
"111"
*/
#include<stdio.h>
#include<string>
using namespace std;
string change_base(int x,int base){
    string out="";
    while (x>0)
    {
        out=to_string(x%base)+out;
        x=x-base;
    }
    return out;
}
```

# Instruction

Fix bugs in change_base.

# Prompt

/*
Change numerical base of input number x to base.
return string representation after the conversion.
base numbers are less than 10.
>>> change_base(8, 3)
"22"
>>> change_base(8, 2)
"1000"
>>> change_base(7, 2)
"111"
*/
#include<stdio.h>
#include<string>
using namespace std;
string change_base(int x,int base){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
