**is_square_free: A simple library to check if a u128 is square free** 

Provides functions to check if a positive integer is square free. 
The concurrent version does not notify all threads if one of them determines that the integer is not square free. We have also not optimized our functions 
using number theoretical methods beyond what is immediately obvious. 

## Running the binary 
One can check if a number of the form `x^y -z`, where x and z are 32-bit integers and y an unsigned 32-bit integer, is square free by running `cargo run --release x y z`. 


