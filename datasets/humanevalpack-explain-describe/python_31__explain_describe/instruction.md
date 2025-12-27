# Context

You are given a reference implementation (canonical solution) to explain.

```python
def is_prime(n):
    if n < 2:
        return False
    for k in range(2, n - 1):
        if n % k == 0:
            return False
    return True
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



def is_prime(n):
    """Return true if a given number is prime, and false otherwise.
    >>> is_prime(6)
    False
    >>> is_prime(101)
    True
    >>> is_prime(11)
    True
    >>> is_prime(13441)
    True
    >>> is_prime(61)
    True
    >>> is_prime(4)
    False
    >>> is_prime(1)
    False
    """


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
