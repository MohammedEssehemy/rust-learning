pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
        .chain(
            list.first()
                .map(|word| vec![format!("And all for the want of a {word}.")])
                .unwrap_or(vec![]),
        )
        .collect::<Vec<_>>()
        .join("\n")
}
