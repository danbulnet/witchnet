use std::time::SystemTime;

pub fn timeit<F: FnMut() -> T, T>(message: &str, mut f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("{}: {:.3} seconds", message, duration.as_millis() as f64 / 1000f64);
    result
}

mod tests {
    #[test]
    fn timeit() {
        let res = super::timeit("test", || {
            let mut res = 0usize;
            for i in 1..10_000 { res += i }
            res
        });
        assert_eq!(res, 49995000usize);
    }
}