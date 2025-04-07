fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    (3..=limit)
        .step_by(2)
        .all(|i| n % i != 0)
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 16, 17, 19, 21, 23, 29, 30];
    for &n in &numbers {
        println!("{:>2} -> {}", n, is_prime(n));
    }
}
