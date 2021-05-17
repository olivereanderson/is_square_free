use std::env;
use std::time::SystemTime;
fn main() {
    let args: Vec<String> = env::args().collect();
    let base = args[1].parse::<i32>().unwrap();
    let exponent = args[2].parse::<i32>().unwrap();
    let sub = args[3].parse::<i32>().unwrap();
    print!("{}^{} - {} is ... \r\n", base, exponent, sub);
    let now = SystemTime::now();
    let input_number = is_square_free::convert_input(base, exponent, sub);
    let input_number_is_square_free = if input_number > 2_u128.pow(60) {
        let num_cpus = num_cpus::get() as u32;
        is_square_free::concurrent_is_square_free(input_number, num_cpus)
    } else {
        is_square_free::is_square_free(input_number)
    };
    if input_number_is_square_free {
        print!(" square free! \r\n");
    } else {
        print!(" not square free! \r\n");
    }
    print!("time elapsed: {:?} \r\n", now.elapsed().unwrap());
}
