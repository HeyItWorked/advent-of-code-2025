use std::fs;

fn solve(input_file: &str) -> i32 {
    /*
    Solve Day 1: Secret Entrance

    Args:
        input_file: Path to the input file containing rotation instructions

    Returns:
        The password (number of times the dial lands on 0)
    */

    // &contents = "R50\nL50\nR10\n"
    let contents = fs::read_to_string(input_file).expect("Failed to read file");

    // lines() split string by newlines and return an iterator
    // rust closure (||) == python lambda
    let steps: Vec<(char, i32)> = contents.lines()
        .iter()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let distance: i32 = line[1..].parse().unwrap();

            (direction, distance)
        })
        .collect();
    
    let count = steps
        .into_iter()
        .fold((50, 0), |(pos, count), (dir, dist)|{
            let sign = if dir == 'L' {-dist} else {dist};
            let new_post = (pos + sign) % 100;
            let new_count = if new_post == 0 {count + 1} else {count};
            (new_post, new_count)
        })
        .1; // extract second element of tuple
    count
}

// ============================================================
// CHANGE THIS NUMBER TO SELECT WHICH TEST TO RUN (0-3)
// 0 = simple, 1 = small, 2 = example, 3 = full
// ============================================================
const SELECTED_TEST: usize = 3;
// ============================================================

struct TestCase {
    name: &'static str,
    path: &'static str,
    expected: Option<i32>,
}

const TEST_CASES: [TestCase; 4] = [
    TestCase {
        name: "simple",
        path: "/home/liamnguyenn98/advent-of-code-2025/day01/tests/test_simple.txt",
        expected: Some(1),
    },
    TestCase {
        name: "small",
        path: "/home/liamnguyenn98/advent-of-code-2025/day01/tests/test_small.txt",
        expected: Some(1),
    },
    TestCase {
        name: "example",
        path: "/home/liamnguyenn98/advent-of-code-2025/day01/tests/test_example.txt",
        expected: Some(3),
    },
    TestCase {
        name: "full",
        path: "/home/liamnguyenn98/advent-of-code-2025/day01/input.txt",
        expected: None,
    },
];

fn run_test(test_num: usize) {
    if test_num >= TEST_CASES.len() {
        println!("Invalid test number: {}", test_num);
        println!("Available tests: 0=simple, 1=small, 2=example, 3=full");
        return;
    }

    let test = &TEST_CASES[test_num];
    println!("\n{}", "=".repeat(50));
    println!("Running: {} (test {})", test.name, test_num);
    println!("File: {}", test.path);
    println!("Expected: {}", match test.expected {
        Some(val) => val.to_string(),
        None => "Unknown".to_string(),
    });
    println!("{}", "=".repeat(50));

    let result = solve(test.path);
    println!("Got:      {}", result);

    if let Some(expected) = test.expected {
        let status = if result == expected { "✓ PASS" } else { "✗ FAIL" };
        println!("Status:   {}", status);
    }
    println!();
}

fn main() {
    run_test(SELECTED_TEST);
}

//   To compile and run:
//  cd /advent-of-code-2025/day01/solution
//  rustc solve.rs -o solve
//  ./solve
