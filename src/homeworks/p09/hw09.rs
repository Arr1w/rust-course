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

#[test]
fn test() {
   let s = "abcdefgh";
   let shifts = [
       (0,  "abcdefgh"),
       (8,  "abcdefgh"),
       (-8, "abcdefgh"),
       (1,  "habcdefg"),
       (2,  "ghabcdef"),
       (10, "ghabcdef"),
       (-1, "bcdefgha"),
       (-2, "cdefghab"),
       (-10,"cdefghab"),
   ];


   shifts
       .iter()
       .for_each(|(n, exp)|
           assert_eq!(
               rotate2(s, n), 
               exp.to_string()
           )
       );
}
