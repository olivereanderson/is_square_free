**is_square_free: A simple library to check if a number is square free** 

# Setup 
This library depends on the [rug crate](https://crates.io/crates/rug) which again depends on the [gmp-mpfr-sys crate](https://crates.io/crates/gmp-mpfr-sys) which needs some setup to build. See the [documentation](https://docs.rs/gmp-mpfr-sys/1.4.5/gmp_mpfr_sys/) on how to install the necessary additional libraries.

# Basic usage

## Check that a 128-bit unsigned integer is square free: 

```rust
use is_square_free;

assert!(is_square_free::fixed_size::is_square_free(42));
```

## Check that a large 128-bit unsigned integer is square free using threads:
```rust 
use is_square_free;

let num_threads = 4;

assert!(is_square_free::fixed_size::concurrent_is_square_free(2u128.pow(61) - 1), num_threads);
```

## Check that an arbitrary precision integer is square free
```rust 
use is_square_free;
use rug::Integer; 

assert!(is_square_free::arbitrary_precision::is_square_free(Integer::from(42)));
```

## Check that a large arbitrary precision integer is square free using threads:
```rust 
use is_square_free 
use rug::Integer;

let num_threads = 4;
assert!(is_square_free::arbitrary_precision::concurrent_is_square_free(Integer::from(2u128.pow(61) -1)));
```

## Running the binary 
One can check if a number of the form `n = x^y -z`, where x and z are 32-bit integers and y an unsigned 32-bit integer, is square free by running `cargo run --release x y z`. If the resulting number `n` is less than `2^60` then a single thread is used, otherwise the number of logical cores on the system will be utilised.  


# Limitations 
As of now we do not use any number theoretical methods apart from some very obvious observations to increase efficiency. Furthermore the concurrent versions do not broadcast to the other threads when one of them has verified that the number is not square free. 