# Context

```python
def largest_divisor(n: int) -> int:
    """ For a given number n, find the largest number that divides n evenly, smaller than n
    >>> largest_divisor(15)
    5
    """
    for i in reversed(range(n)):
        if n - i == 0:
            return i
```

# Instruction

Fix bugs in largest_divisor.

# Prompt



def largest_divisor(n: int) -> int:
    """ For a given number n, find the largest number that divides n evenly, smaller than n
    >>> largest_divisor(15)
    5
    """


# Instructions

Implement your solution in `solution/solution.py`.
Ensure your submission is self-contained and compiles/runs correctly.

```
