# Context

```python
def prime_length(string):
    """Write a function that takes a string and returns True if the string
    length is a prime number or False otherwise
    Examples
    prime_length('Hello') == True
    prime_length('abcdcba') == True
    prime_length('kittens') == True
    prime_length('orange') == False
    """
    l = len(string)
    if l == 0 or l == 1:
        return False
    for i in range(3, l):
        if l % i == 0:
            return False
    return True
```

# Instruction

Fix bugs in prime_length.

# Prompt


def prime_length(string):
    """Write a function that takes a string and returns True if the string
    length is a prime number or False otherwise
    Examples
    prime_length('Hello') == True
    prime_length('abcdcba') == True
    prime_length('kittens') == True
    prime_length('orange') == False
    """


# Instructions

Implement your solution in `solution/solution.py`.
Ensure your submission is self-contained and compiles/runs correctly.

```
