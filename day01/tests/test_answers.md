# Test Case Answers

## test_simple.txt
**Expected Output:** 0

**Walkthrough:**
- Start: position = 50
- R50: position = (50 + 50) % 100 = 0 ✗ (Wait, this lands on 0!)
- Actually: position = 0, zeros_count = 1

**Expected Output:** 1

**Why:** After the first and only rotation, the dial points at 0, so count = 1.

---

## test_small.txt
**Expected Output:** 1

**Walkthrough:**
- Start: position = 50
- R50: position = (50 + 50) % 100 = 0 ✓ → zeros_count = 1
- L50: position = (0 - 50) % 100 = 50 (wraps around)
- R10: position = (50 + 10) % 100 = 60

**Final position:** 60, **zeros_count:** 1

---

## test_example.txt
**Expected Output:** 3

**Walkthrough:**
This is the official example from the problem statement.

- Start: position = 50
- L68: (50 - 68) % 100 = 82
- L30: (82 - 30) % 100 = 52
- R48: (52 + 48) % 100 = 0 ✓ → zeros_count = 1
- L5: (0 - 5) % 100 = 95
- R60: (95 + 60) % 100 = 55
- L55: (55 - 55) % 100 = 0 ✓ → zeros_count = 2
- L1: (0 - 1) % 100 = 99
- L99: (99 - 99) % 100 = 0 ✓ → zeros_count = 3
- R14: (0 + 14) % 100 = 14
- L82: (14 - 82) % 100 = 32

**Final position:** 32, **zeros_count:** 3

---

## How to Test

```bash
# Test simple case
python3 solution.py < test_simple.txt
# Expected: 1

# Test small case
python3 solution.py < test_small.txt
# Expected: 1

# Test example case
python3 solution.py < test_example.txt
# Expected: 3

# Test full input
python3 solution.py < input.txt
# Should give your answer
```

Or if you write your solution to take a filename as an argument:

```bash
python3 solution.py test_simple.txt
python3 solution.py test_small.txt
python3 solution.py test_example.txt
python3 solution.py input.txt
```
