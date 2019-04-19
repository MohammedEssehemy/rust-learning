use std::collections::HashMap;
pub fn log() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
