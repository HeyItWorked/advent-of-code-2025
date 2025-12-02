# Day 1: Secret Entrance

## Problem

The Elves have good news and bad news. The good news is that they've discovered project management! The bad news is that they've realized they have a different emergency: none of them have any time left to decorate the North Pole!

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed. A document taped to the wall explains:

"Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an **L** or **R** which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

### Examples

- If the dial were pointing at 11, a rotation of **R8** would cause the dial to point at 19
- After that, a rotation of **L19** would cause it to point at 0

Because the dial is a circle:
- Turning the dial left from 0 one click makes it point at 99
- Turning the dial right from 99 one click makes it point at 0

### Key Rules

- The dial starts by pointing at **50**
- The actual password is the **number of times the dial is left pointing at 0** after any rotation in the sequence

### Example Walkthrough

Given these rotations:
```
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
```

The dial moves as follows:
1. Starts at 50
2. L68 → 82
3. L30 → 52
4. R48 → 0 ✓
5. L5 → 95
6. R60 → 55
7. L55 → 0 ✓
8. L1 → 99
9. L99 → 0 ✓
10. R14 → 14
11. L82 → 32

The dial points at 0 a total of **3 times**, so the password is **3**.

## Task

Analyze the rotations in your attached document. What's the actual password to open the door?
