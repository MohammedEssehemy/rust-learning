mod cacher;
mod counter;
mod iter;
mod shoe;

fn _take_owner(_vec: Vec<i32>) {}
fn _try_capture_env() {
    let data = vec![1, 2, 3];

    let equal_to_x = move || {
        println!("captured {data:?} by value");
        // _take_owner(data);
    };
    equal_to_x();
    equal_to_x();
    // println!("{data:?}"); // fail since data is moved into closure
}

fn main() {
    _try_capture_env();
    cacher::_try_cacher();
    iter::_try_iter();
    shoe::_try_shoe();
    counter::_try_counter();
}
