# Sudoku Validator

Just some fun with Rust threads and validating Sudoku puzzles.

## Usage

```
cargo run -- < puzzle-invalid.input
```

## Input

Space separated rows of numbers. For example:

```
1 1 1 1 1 1 1 1 9
2 1 1 7 2 1 6 3 8
3 3 1 6 1 1 2 9 7
4 1 1 8 6 1 7 2 6
5 5 1 2 4 1 3 6 5
6 6 1 3 9 1 4 5 4
7 7 1 9 5 1 8 4 3
8 9 1 1 8 1 5 7 2
9 1 1 1 1 1 1 1 1
```


## Output

The rows, columns and regions that are invalid and the total number of invalid checks:

```
row 0 is invalid
row 1 is invalid
row 2 is invalid
row 3 is invalid
row 4 is invalid
row 5 is invalid
row 6 is invalid
row 7 is invalid
row 8 is invalid
column 1 is invalid
column 2 is invalid
column 3 is invalid
column 4 is invalid
column 6 is invalid
column 5 is invalid
column 7 is invalid
region 0 is invalid
region 1 is invalid
region 2 is invalid
region 3 is invalid
region 4 is invalid
region 7 is invalid
region 6 is invalid
region 8 is invalid
region 5 is invalid
Errors found: 25

```
