# Flat Matrix

this is a library that adds flat matrices


```rust
use FlatMatrix::FlatMatrix;

fn main() {
    let matrix: FlatMatrix<bool> = FlatMatrix::new(10, 20, false);
    println!("{}", matrix.get(4, 4).unwrap()); //false
    matrix.set(4, 4, true);
    println!("{}", matrix.get(4, 4).unwrap()); //true
}

```

