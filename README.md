# gpt3_macro
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/vongaisberg/gpt3_macro/build)
![Crates.io](https://img.shields.io/crates/d/gpt3_macro)
![Lines of code](https://img.shields.io/tokei/lines/github/vongaisberg/gpt3_macro)
![Crates.io](https://img.shields.io/crates/l/gpt3_macro)
![Crates.io](https://img.shields.io/crates/v/gpt3_macro)

Rust macro that uses GPT3 codex to generate code at compiletime.

Just describe what you want the function to do and (optionally) define a function header. The macro will generate the sourcecode for you at compiletime.

It can also generate tests for you. (See example 3)

## Example 1
```rust
create_function!("checks if number is prime" fn is_prime(num: u64) -> bool);
```
will (usually) expand to something like:

```rust
// A rust function that checks if number is prime
fn is_prime(num: u64) -> bool {
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
```

## Example 2
```rust
create_function!("prints n elements of the fibonnacci sequence to stdout" fn fib(n: u64));
```
sometimes expands to:

```rust
// prints n elements of the fibonnacci sequence to stdout
fn fib (n : u64) {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
        println!("{}", c);
    }
}
```

## Example 3: Code and test generation
```rust
create_function_and_tests!("fizzbuzz", fn fizzbuzz(n: u64) -> String)
```
will often expand to:
```rust

// A rust function that fizzbuzz
fn fizzbuzz (n : u64) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}
// 5 tests for the function
#[test]
fn test_fizzbuzz_1() {
    assert_eq!(fizzbuzz(1), "1");
}
#[test]
fn test_fizzbuzz_2() {
    assert_eq!(fizzbuzz(2), "2");
}
#[test]
fn test_fizzbuzz_3() {
    assert_eq!(fizzbuzz(3), "Fizz");
}
#[test]
fn test_fizzbuzz_4() {
    assert_eq!(fizzbuzz(4), "4");
}
#[test]
fn test_fizzbuzz_5() {
    assert_eq!(fizzbuzz(5), "Buzz");
}
```

## Pros and Cons
| Pros | Cons |
| ---- | ---- |
| Spend less time coding simple utility functions and save your brainpower for the big problems | Compilation takes way longer |
| Create more readable sourcecode — the documentation IS the sourcode. | You need to be part of the GPT3 Codex private beta
| A little nondeterminism during compilation is fun! | GPT3 Codex will not always be free :(|

## Installation

Generate an OpenAI API key at the [OpenAI Account Page](https://beta.openai.com/account/api-keys) and set the ```$OPENAI_KEY``` environment variable

Then execute
```
cargo add gpt3_macro
```
or manually add 
```
gpt3_macro = "0.3.1"
```
to your ```Cargo.toml```

