**is_square_free: A simple library to check if a u128 is square free** 

Provides a function `is_square_free(n: u128) -> bool` and also a concurrent version `concurrent_is_square_free(n: u128, num_threads: u32) -> bool`. 
The concurrent version does not notify all threads if one of them determines that the integer is not square free. We have also not optimized our functions 
using number theoretical methods beyond what is immediately obvious. 

## Running the binary 
One can check if a u128 of the form `x^y -z` is square free by running `cargo run --release x y z`. 


