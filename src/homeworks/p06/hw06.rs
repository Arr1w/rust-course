fn main() {
    let triangle_count = 5; // Кількість трикутників

    let max_width = triangle_count * 2 + 1;

    // Генерація трикутників
    for triangle in 0..triangle_count {
        for line in 0..=triangle {
            let stars = 1 + line * 2;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }

    // Стовбур (2 рядки, по 1 зірочці)
    for _ in 0..2 {
        let spaces = (max_width - 1) / 2;
        println!("{}*", " ".repeat(spaces));
    }
}
