pub fn log() {
    log_str("السلام عليكم");
    log_str("Dobrý den");
    log_str("Hello");
    log_str("שָׁלוֹם");
    log_str("नमस्ते");
    log_str("こんにちは");
    log_str("안녕하세요");
    log_str("你好");
    log_str("Olá");
    log_str("Здравствуйте");
    log_str("Hola");
}

fn log_str(word: &str) {
    let hello = String::from(word);
    println!("{}", hello);
}
