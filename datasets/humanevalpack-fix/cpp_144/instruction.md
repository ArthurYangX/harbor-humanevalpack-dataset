# Context

```cpp
/*
Your task is to implement a function that will simplify the expression
x * n. The function returns true if x * n evaluates to a whole number and false
otherwise. Both x and n, are string representation of a fraction, and have the following format,
<numerator>/<denominator> where both numerator and denominator are positive whole numbers.

You can assume that x, and n are valid fractions, and do not have zero as denominator.

simplify("1/5", "5/1") = true
simplify("1/6", "2/1") = false
simplify("7/10", "10/2") = false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool simplify(string x,string n){
    int a,b,c,d,i;
    for (i=0;i<x.size();i++)
        if (x[i]=='/') 
        {
            a=atoi(x.substr(0,i).c_str());
            b=atoi(x.substr(i+1).c_str());
        }
    for (i=0;i<n.size();i++)
        if (n[i]=='/') 
        {
            c=atoi(n.substr(0,i).c_str());
            d=atoi(n.substr(i+1).c_str());
            a=atoi(n.substr(0,i).c_str());
            b=atoi(n.substr(i+1).c_str());
        }
    if ((a*c)%(b*d)==0) return true;
    return false;
}
```

# Instruction

Fix bugs in simplify.

# Prompt

/*
Your task is to implement a function that will simplify the expression
x * n. The function returns true if x * n evaluates to a whole number and false
otherwise. Both x and n, are string representation of a fraction, and have the following format,
<numerator>/<denominator> where both numerator and denominator are positive whole numbers.

You can assume that x, and n are valid fractions, and do not have zero as denominator.

simplify("1/5", "5/1") = true
simplify("1/6", "2/1") = false
simplify("7/10", "10/2") = false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool simplify(string x,string n){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
