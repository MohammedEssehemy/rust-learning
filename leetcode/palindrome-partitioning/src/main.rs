use palindrome_partitioning::Solution;
fn main() {
    println!("{:?}", Solution::partition("aab".to_string()));
    println!("{:?}", Solution::partition("aabb".to_string()));
}
