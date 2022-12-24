pub fn raindrops(n: u32) -> String {
    let sound = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|(number, sound)| {
            if n % number == 0 {
                Some(sound.to_string())
            } else {
                None
            }
        })
        .collect::<String>();

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}
