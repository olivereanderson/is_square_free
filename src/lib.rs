use crossbeam_channel::{bounded};
use std::{iter, u128};
use bus::{BusReader,Bus};
//use crossbeam_utils::thread::scope;

/// Check if a number is square free
///
/// Panics if n is greater than the largest square number less than u128::MAX
pub fn is_square_free(n: u128) -> bool {
    const MAX: u128 = 2_u128.pow(126);
    if n > MAX {
        panic!("cannot work with numbers larger that {} \r\n", MAX);
    }
    // Enough to test 4 followed by all odd squares that are less than n.
    !iter::once(2_u128)
        .chain((3..n).into_iter().step_by(2))
        .map(|i| i.pow(2))
        .take_while(|i| i <= &n)
        .any(|i| (n % i) == 0)
}

fn is_square_free_piece(n: u128, from: u128, to: u128, mut r: BusReader<bool>) -> bool {
    if from <= 2 && ((n % 4) == 0) {
        return false;
    }
    !(from..to)
        .into_iter()
        .map(|i| i.pow(2))
        .take_while(|i| (i <= &n) && (r.try_recv().is_err()))
        .any(|i| (n % i) == 0)
}


pub fn concurrent_is_square_free(n: u128, num_threads: u32) -> bool {
    let num_threads = num_threads as u128; 
    // multiple producer single receiver 
    let (sender, receiver) = bounded(0);
    // single producer multiple receiver 
    let mut tx = Bus::new(1);
    // split the computation into a piece per given thread 
    let iter = (1_u128..=num_threads).into_iter();
    let elements_per_thread = n/(num_threads);
    for i in iter {
        let from = (i-1) *elements_per_thread + 2;
        let to = if i != num_threads {i * elements_per_thread + 2}else {n};
        let rx = tx.add_rx(); 
        let sender_clone = sender.clone();
        std::thread::spawn(move || {
            if !is_square_free_piece(n, from, to, rx ){
                sender_clone.send(false).unwrap();
            }
        });
    }
    drop(sender);

    if receiver.recv().is_ok(){
        if tx.try_broadcast(false).is_err() {
            print!("Failed to broadcast \r\n");
        }
        false 
    } else {
        true
    }
}

pub fn convert_input(base: i32, exponent: i32, sub: i32) -> u128 {
    u128::from(base.abs() as u32).pow(exponent.abs() as u32) - u128::from(sub.abs() as u32)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn square_free_concurrent(){
        let num_threads = 8;
        assert!(concurrent_is_square_free(21,num_threads));
        assert!(concurrent_is_square_free(2,num_threads));
        assert!(concurrent_is_square_free(26, num_threads));
    }

    #[test]
    fn not_square_free_concurrent() {
        let num_threads = 8; 
        assert!(!concurrent_is_square_free(4,num_threads));
        assert!(!concurrent_is_square_free(9, num_threads));
        assert!(!concurrent_is_square_free(144,num_threads));
    }
    #[test]
    fn not_square_free() {
        assert!(!is_square_free(4));
        assert!(!is_square_free(9));
        assert!(!is_square_free(144));
    }

    #[test]
    fn square_free() {
        assert!(is_square_free(2));
        assert!(is_square_free(21));
        assert!(is_square_free(2_u128.pow(5) - 1));
    }
}
