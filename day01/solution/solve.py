def solve(input_file: str) -> int:
    """
    Solve Day 1: Secret Entrance

    Args:
        input_file: Path to the input file containing rotation instructions

    Returns:
        The password (number of times the dial lands on 0)
    """
    # print(f"")
    with open(input_file, 'r') as f:
        lines = f.readlines()
        steps = [(line.strip()[0], int(line.strip()[1:])) for line in lines]

        init_pos = 50
        count = 0
        
        for dir, dist in steps:
            sign = 1 if dir == 'R' else -1
            new_pos = (init_pos + sign*dist) % 100
            if new_pos == 0:
                count = count + 1
            init_pos = new_pos
    return count

# ============================================================
# CHANGE THIS NUMBER TO SELECT WHICH TEST TO RUN (0-3)
# 0 = simple, 1 = small, 2 = example, 3 = full
# ============================================================
SELECTED_TEST = 3
# ============================================================

TEST_CASES = {
    0: {
        "name": "simple",
        "path": "/home/liamnguyenn98/advent-of-code-2025/day01/tests/test_simple.txt",
        "expected": 1,
    },
    1: {
        "name": "small",
        "path": "/home/liamnguyenn98/advent-of-code-2025/day01/tests/test_small.txt",
        "expected": 1,
    },
    2: {
        "name": "example",
        "path": "/home/liamnguyenn98/advent-of-code-2025/day01/tests/test_example.txt",
        "expected": 3,
    },
    3: {
        "name": "full",
        "path": "/home/liamnguyenn98/advent-of-code-2025/day01/input.txt",
        "expected": None,  # Unknown until solved
    },
}


def run_test(test_num: int):
    """Run a specific test case and show results."""
    if test_num not in TEST_CASES:
        print(f"Invalid test number: {test_num}")
        print(f"Available tests: 0=simple, 1=small, 2=example, 3=full")
        return

    test = TEST_CASES[test_num]
    print(f"\n{'='*50}")
    print(f"Running: {test['name']} (test {test_num})")
    print(f"File: {test['path']}")
    print(f"Expected: {test['expected']}")
    print(f"{'='*50}")

    result = solve(test["path"])
    print(f"Got:      {result}")

    if test["expected"] is not None:
        match = "✓ PASS" if result == test["expected"] else "✗ FAIL"
        print(f"Status:   {match}")
    print()


if __name__ == "__main__":
    run_test(SELECTED_TEST)
