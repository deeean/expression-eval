# expression-parser

```rust
fn main() {
  match Evaluator::evaluate("2 ** 10 / 16 * 32 - (1024 + 512)") {
    Ok(result) => println!("{}", result),
    Err(error) => println!("{}", error),
  }
}
```