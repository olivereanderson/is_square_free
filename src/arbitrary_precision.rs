pub use concurrent_implementation::concurrent_is_square_free;
use rug::Integer;

pub fn is_square_free(n: Integer) -> bool {
    let n = n.abs(); // In case a negative integer is passed in.
                     // First check if n is divisible by 4, then check if it is divisible by any odd square <= n.
    if n.is_divisible(&Integer::from(4)) {
        false
    } else {
        let mut m = Integer::from(3);
        let mut m_square = Integer::from(m.square_ref());
        while m_square <= n {
            if n.is_divisible(&m_square) {
                return false;
            }
            m += 2;
            m_square = Integer::from(m.square_ref());
        }
        true
    }
}

pub mod concurrent_implementation {
    use rug::Integer;
    use std::ops::Div;
    fn is_square_free_piece(n: &Integer, from: Integer, to: Integer) -> bool {
        let mut m = from;
        let mut m_square = Integer::from(m.square_ref());
        let end = to.square();
        while m_square <= end {
            if n.is_divisible(&m_square) {
                return false;
            }
            m += 2;
            m_square = Integer::from(m.square_ref());
        }
        true
    }

    fn is_divisible_by_four(n: &Integer) -> bool {
        (n > &Integer::from(2)) && n.is_power_of_two()
    }

    pub fn concurrent_is_square_free(n: Integer, num_threads: u32) -> bool {
        let n = n.abs();
        if n <= Integer::from(3) {
            return true;
        }
        let i_sqrt = Integer::from(n.sqrt_ref());
        (!is_divisible_by_four(&n))
            && concurrent_is_square_free_start(
                n.clone(),
                Integer::from(3),
                num_threads,
                i_sqrt.clone(),
            )
            && (n != i_sqrt.square())
    }

    fn concurrent_is_square_free_start(
        n: Integer,
        start: Integer,
        num_threads: u32,
        i_sqrt: Integer,
    ) -> bool {
        let elements_per_thread = (i_sqrt.clone() - start.clone()).div(num_threads);
        let (tx, rx) = std::sync::mpsc::channel();
        for i in (1..=num_threads).into_iter() {
            let from = start.clone() + (i - 1) * elements_per_thread.clone();
            let to = if i != num_threads {
                start.clone() + i * elements_per_thread.clone()
            } else {
                i_sqrt.clone()
            };
            let tx_clone = tx.clone();
            let n_clone = n.clone();
            std::thread::spawn(move || {
                if !is_square_free_piece(&n_clone, from, to) {
                    let _ = tx_clone.send(false);
                }
                drop(tx_clone);
            });
        }
        drop(tx);
        rx.recv().is_err()
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        #[test]
        fn square_free_concurrent() {
            let num_threads = 8;
            assert!(concurrent_is_square_free(Integer::from(21), num_threads));
            assert!(concurrent_is_square_free(Integer::from(2), num_threads));
            assert!(concurrent_is_square_free(Integer::from(26), num_threads));
        }

        #[test]
        fn not_square_free_concurrent() {
            let num_threads = 8;
            assert!(!concurrent_is_square_free(Integer::from(4), num_threads));
            assert!(!concurrent_is_square_free(Integer::from(9), num_threads));
            assert!(!concurrent_is_square_free(Integer::from(144), num_threads));
        }
    }
}

pub fn convert_input(base: i32, exponent: i32, sub: i32) -> Integer {
    Integer::from(Integer::i_pow_u(base, exponent.abs() as u32)) - sub
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn not_square_free() {
        assert!(!is_square_free(Integer::from(4)));
        assert!(!is_square_free(Integer::from(9)));
        assert!(!is_square_free(Integer::from(144)));
    }

    #[test]
    fn square_free() {
        assert!(is_square_free(Integer::from(2)));
        assert!(is_square_free(Integer::from(21)));
        assert!(is_square_free(Integer::from(2_u128.pow(5) - 1)));
    }
}
