# Context

You are given a reference implementation (canonical solution) to explain.

```python
def change_base(x: int, base: int):
    ret = ""
    while x > 0:
        ret = str(x % base) + ret
        x //= base
    return ret
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



def change_base(x: int, base: int):
    """Change numerical base of input number x to base.
    return string representation after the conversion.
    base numbers are less than 10.
    >>> change_base(8, 3)
    '22'
    >>> change_base(8, 2)
    '1000'
    >>> change_base(7, 2)
    '111'
    """


# Instructions

- Write the explanation to `solution/explanation.txt`.
- Do not modify tests.
