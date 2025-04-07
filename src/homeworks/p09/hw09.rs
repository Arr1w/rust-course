fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }

    // Нормалізуємо зсув у діапазон 0..len
    let n = ((n % len) + len) % len;
    let n = n as usize;

    let (left, right) = s.split_at(s.len() - n);
    format!("{}{}", right, left)
}
