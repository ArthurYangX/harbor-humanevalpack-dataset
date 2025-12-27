# Context

You are given a reference implementation (canonical solution) to explain.

```python
def below_threshold(l: list, t: int):
    for e in l:
        if e >= t:
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



def below_threshold(l: list, t: int):
    """Return True if all numbers in the list l are below threshold t.
    >>> below_threshold([1, 2, 4, 10], 100)
    True
    >>> below_threshold([1, 20, 4, 10], 5)
    False
    """


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
