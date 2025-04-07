fn draw_tree(levels: usize) {
    let max_width = levels * 2 + 1;

    (1..=levels).for_each(|level| {
        (1..=level + 1).for_each(|line| {
            let stars = line * 2 - 1;
            let padding = (max_width - stars) / 2;
            let line_str: String = " ".repeat(padding) + &"*".repeat(stars);
            println!("{}", line_str);
        });
    });

    // Стовбур
    let trunk_width = 1;
    let trunk_height = 1;
    let padding = (max_width - trunk_width) / 2;
    for _ in 0..trunk_height {
        println!("{}{}", " ".repeat(padding), "|");
    }
}

fn main() {
    let triangles = 5; // Можна змінити кількість рівнів тут
    draw_tree(triangles);
}
