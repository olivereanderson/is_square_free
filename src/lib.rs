pub use concurrent_implementation::concurrent_is_square_free;
use std::{iter, u128};
/// Check if a number is square free
///
/// Panics if n is greater than 2^126
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

pub mod concurrent_implementation {
    use integer_sqrt::IntegerSquareRoot;
    fn is_square_free_piece(n: u128, from: u128, to: u128) -> bool {
        !(from..to)
            .into_iter()
            .step_by(2)
            .map(|i| i.pow(2))
            .take_while(|i| (i <= &n))
            .any(|i| (n % i) == 0)
    }

    fn is_square_free_edge_cases(n: u128, i_sqrt: u128) -> bool {
        !(i_sqrt >= 2 && ((n % 4) == 0 || (n == i_sqrt.pow(2))))
    }

    /// Check if a number is square free
    ///
    /// Panics if n is creater than 2^126
    pub fn concurrent_is_square_free(n: u128, num_threads: u32) -> bool {
        const MAX: u128 = 2_u128.pow(126);
        if n > MAX {
            panic!("cannot work with numbers larger that {} \r\n", MAX);
        }
        let num_threads = num_threads as u128;
        let i_sqrt = n.integer_sqrt();
        let elements_per_thread = i_sqrt / (num_threads);
        if !is_square_free_edge_cases(n, i_sqrt) {
            return false;
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        for i in (1_u128..=num_threads).into_iter() {
            let from = (i - 1) * elements_per_thread + 2;
            let to = if i != num_threads {
                i * elements_per_thread + 2
            } else {
                i_sqrt
            };
            let sender_clone = sender.clone();
            std::thread::spawn(move || {
                if !is_square_free_piece(n, from, to) {
                    let _ = sender_clone.send(false);
                }
                drop(sender_clone);
            });
        }
        drop(sender);

        receiver.recv().is_err() // transmitters only send messages if n is not square free
    }
    #[cfg(test)]
    mod tests {

        use super::*;
        #[test]
        fn square_free_concurrent() {
            let num_threads = 8;
            assert!(concurrent_is_square_free(21, num_threads));
            assert!(concurrent_is_square_free(2, num_threads));
            assert!(concurrent_is_square_free(26, num_threads));
        }

        #[test]
        fn not_square_free_concurrent() {
            let num_threads = 8;
            assert!(!concurrent_is_square_free(4, num_threads));
            assert!(!concurrent_is_square_free(9, num_threads));
            assert!(!concurrent_is_square_free(144, num_threads));
        }
    }
}

pub fn convert_input(base: i32, exponent: i32, sub: i32) -> u128 {
    u128::from(base.abs() as u32).pow(exponent.abs() as u32) - u128::from(sub.abs() as u32)
}

#[cfg(test)]
mod tests {

    use super::*;
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
