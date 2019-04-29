use figlet::font::Font;

fn main() {
    println!("Hello, world!");
    let f = Font::load_font("4Max.flf");
    dbg!(f);
}
