# Context

```python
def starts_one_ends(n):
    """
    Given a positive integer n, return the count of the numbers of n-digit
    positive integers that start or end with 1.
    """
    if n == 1: return 1
    return 18 * n * (10 ** (n - 2))
```

# Instruction

Fix bugs in starts_one_ends.

# Prompt


def starts_one_ends(n):
    """
    Given a positive integer n, return the count of the numbers of n-digit
    positive integers that start or end with 1.
    """


# Instructions

Implement your solution in `solution/solution.py`.
Ensure your submission is self-contained and compiles/runs correctly.

```
