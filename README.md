# Data Analysis Tool

## Overview

This project involves the implementation of a front-end compiler in Rust for a custom Data-Analysis (DA) language. The project focuses on developing a Lexical Analyzer (Scanner) and a Syntax Analyzer (Parser) to process DA language programs. These components work together to identify and report lexical and syntax errors in the input program and generate output based on the successful analysis.

## Project Structure

### Lexical Analyzer (Scanner)

The Lexical Analyzer scans the input DA program and generates a stream of tokens. It recognizes various elements such as identifiers, numbers, keywords, and symbols based on the DA grammar. The tokens produced are then passed to the Syntax Analyzer for further processing. The scanner adheres to the "hide the head in the sand" error-handling approach, where the process halts immediately upon encountering the first lexical error.

### Syntax Analyzer (Parser)

The Syntax Analyzer takes the token stream from the Lexical Analyzer and checks if it adheres to the rules defined in the DA grammar. If the syntax is valid, the analyzer proceeds to the next phase based on the command line flag provided. The parser also follows the "hide the head in the sand" error-handling strategy, stopping at the first detected syntax error.

### Output Generation

Depending on the command line flag provided, the program generates one of the following outputs:

- **Scheme Code** (`-s` flag): The program generates Scheme code, which can be used by a Scheme interpreter to execute the operations defined in the DA program.
- **Prolog Queries** (`-p` flag): The program generates a series of Prolog queries based on the operations specified in the DA program.

### Example Usage

To generate Scheme output:

```bash
cargo run input.da -s
```

Example output:

```scheme
; Processing Input File input.da
; Lexical and Syntax analysis passed
(define xvalues (read-csv "./file.csv" #f 0))
(define yvalues (read-csv "./file.csv" #f 1))
(define a (regressiona xvalues yvalues))
(define b (regressionb xvalues yvalues))
(define r (correlation xvalues yvalues))
(display "value of a = ")
(newline)
(display a)
(newline)
(display "value of b = ")
(newline)
(display b)
(newline)
(display "value of r = ")
(newline)
(display r)
(newline)
```

To generate Prolog output:

```bash
cargo run input.da -p
```

Example output:

```prolog
/* Processing input file input.da
   Lexical and Syntax analysis passed */

main :-
   load_data_column('file.csv', false, 0, Data0),
   load_data_column('file.csv', false, 1, Data1),
   regressiona(Data0, Data1, A),
   regressionb(Data0, Data1, B),
   correlation(Data0, Data1, R),
   writeln("value of a = "),
   writeln(A),
   writeln("value of b = "),
   writeln(B),
   writeln("value of r = "),
   writeln(R).
```

### Grammar

The program is designed to parse a specific grammar that defines the structure of valid DA programs. The tokens include keywords like `data`, `input`, `process`, `output`, along with various operators, identifiers, and types.

### Running the Program

The program is executed via Cargo and requires an input file and a command line flag (`-s` for Scheme or `-p` for Prolog). The input file should contain a DA program written according to the specified grammar.

### Testing

Several test files (`test0.da` to `test5.da`) are included for testing purposes, with some containing deliberate lexical and syntax errors. The program is designed to handle these errors gracefully, reporting the first error encountered and halting further processing.
