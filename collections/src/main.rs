mod hash_maps;
mod helpers;
mod strings;
mod vectors;
fn main() {
    helpers::print_title("Vectors");
    vectors::log();
    helpers::print_separator();
    helpers::print_title("Strings");
    strings::log();
    helpers::print_separator();
    helpers::print_title("Hash Maps");
    hash_maps::log();
    helpers::print_separator();
}
