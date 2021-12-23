# gpt3_macro
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/vongaisberg/gpt3_macro/Rust)
![Crates.io](https://img.shields.io/crates/d/gpt3_macro)
![Lines of code](https://img.shields.io/tokei/lines/github/vongaisberg/gpt3_macro)
![Crates.io](https://img.shields.io/crates/l/gpt3_macro)
![Crates.io](https://img.shields.io/crates/v/gpt3_macro)

Rust macro that uses GPT3 codex to generate code at compiletime.

Just describe what you want the function to do and (optionally) define a function header. The macro will generate the sourcecode for you at compiletime.

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
gpt3_macro = "0.2.2"
```
to your ```Cargo.toml```

