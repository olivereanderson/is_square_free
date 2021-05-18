use std::time::SystemTime;
use std::{convert::TryInto, env};
fn main() {
    let args: Vec<String> = env::args().collect();
    let base = args[1].parse::<i32>().unwrap();
    let exponent: i32 = args[2].parse::<u32>().unwrap().try_into().unwrap();
    let sub = args[3].parse::<i32>().unwrap();
    print!("{}^{} - {} is ... \r\n", base, exponent, sub);
    let num_cpus = num_cpus::get() as u32;
    let now = SystemTime::now();
    let input_number_is_square_free: bool;
    if base.checked_pow(exponent as u32).is_some() {
        let input_number = is_square_free::convert_input(base, exponent, sub);
        input_number_is_square_free = if input_number > 2_u128.pow(60) {
            is_square_free::fixed_size::concurrent_is_square_free(input_number, num_cpus)
        } else {
            is_square_free::fixed_size::is_square_free(input_number)
        };
    } else {
        let input_number = is_square_free::arbitrary_precision::convert_input(base, exponent, sub);
        input_number_is_square_free =
            is_square_free::arbitrary_precision::concurrent_is_square_free(input_number, num_cpus);
    }
    if input_number_is_square_free {
        print!(" square free! \r\n");
    } else {
        print!(" not square free! \r\n");
    }
    print!("time elapsed: {:?} \r\n", now.elapsed().unwrap());
}
