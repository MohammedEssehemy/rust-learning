use oop::screen;
use oop::blog;
use oop::blog_rust_way;

fn main() {
    screen::try_screen();
    println!("-------------------------");
    blog::try_blog();
    println!("-------------------------");
    blog_rust_way::try_blog_rust_way();
}
