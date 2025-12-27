# Context

```cpp
/*
Return true if a given number is prime, and false otherwise.
>>> is_prime(6)
false
>>> is_prime(101)
true
>>> is_prime(11)
true
>>> is_prime(13441)
true
>>> is_prime(61)
true
>>> is_prime(4)
false
>>> is_prime(1)
false
*/
#include<stdio.h>
using namespace std;
bool is_prime(long long n){
    if (n<1) return false;
    for (long long i=1;i*i<=n;i++)
        if (n%i==0) return false;
    return true;
}
```

# Instruction

Fix bugs in is_prime.

# Prompt

/*
Return true if a given number is prime, and false otherwise.
>>> is_prime(6)
false
>>> is_prime(101)
true
>>> is_prime(11)
true
>>> is_prime(13441)
true
>>> is_prime(61)
true
>>> is_prime(4)
false
>>> is_prime(1)
false
*/
#include<stdio.h>
using namespace std;
bool is_prime(long long n){


# Instructions

Implement your solution in `solution/solution.cpp`.
Ensure your submission is self-contained and compiles/runs correctly.

```
