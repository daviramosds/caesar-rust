use std::env;

fn enc(text: &str, key: u8) -> String {
    let upper_text = text.to_uppercase();
    upper_text
        .chars()
        .map(|c| {
            let l = b'A';
            let c_u8 = c as u8;

            return (l + (c_u8 + key - l) % 26) as char
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let text = &args[1];
    
    let enc = enc(text, 3);
    println!("{enc}");
}
