# Context

```cpp
/*
Write a function that takes a string and returns true if the string
length is a prime number or false otherwise
Examples
prime_length("Hello") == true
prime_length("abcdcba") == true
prime_length("kittens") == true
prime_length("orange") == false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool prime_length(string str){
    int l,i;
    l=str.length();
    if (l<2) return false;
    for (i=3;i*i<=l;i++)
    if (l%i==0) return false;
    return true;
}
```

# Instruction

Fix bugs in prime_length.

# Prompt

/*
Write a function that takes a string and returns true if the string
length is a prime number or false otherwise
Examples
prime_length("Hello") == true
prime_length("abcdcba") == true
prime_length("kittens") == true
prime_length("orange") == false
*/
#include<stdio.h>
#include<string>
using namespace std;
bool prime_length(string str){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
