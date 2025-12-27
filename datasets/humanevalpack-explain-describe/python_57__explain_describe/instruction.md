# Context

You are given a reference implementation (canonical solution) to explain.

```python
def monotonic(l: list):
    if l == sorted(l) or l == sorted(l, reverse=True):
        return True
    return False
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



def monotonic(l: list):
    """Return True is list elements are monotonically increasing or decreasing.
    >>> monotonic([1, 2, 4, 20])
    True
    >>> monotonic([1, 20, 4, 10])
    False
    >>> monotonic([4, 1, 0, -10])
    True
    """


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
