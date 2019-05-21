use figlet::font::Font;

fn main() {
    println!("Hello, world!");
    let f = Font::load_font("Avatar.flf");
    println!("{}", f.unwrap().convert("Hello"));
}
