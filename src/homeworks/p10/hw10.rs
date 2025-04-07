fn is_palindrome(x: u32) -> bool {
    if x < 10 {
        return true;
    }
    let s = x.to_string();
    let reversed_s: String = s.chars().rev().collect();
    s == reversed_s
}
