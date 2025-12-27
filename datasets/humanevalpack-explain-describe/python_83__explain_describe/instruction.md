# Context

You are given a reference implementation (canonical solution) to explain.

```python
def starts_one_ends(n):
    if n == 1: return 1
    return 18 * (10 ** (n - 2))
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


def starts_one_ends(n):
    """
    Given a positive integer n, return the count of the numbers of n-digit
    positive integers that start or end with 1.
    """


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
