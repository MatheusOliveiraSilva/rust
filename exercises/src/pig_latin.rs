
pub fn convert_words(string: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u']; // chars, não strings
    
    if let Some(first_char) = string.chars().next() {
        if vowels.contains(&first_char.to_ascii_lowercase()) {
            return string.to_string() + "-hay";
        } else {
            let first_char_str = &first_char.to_string();
            let rest: String = string.chars().skip(1).collect();

            return rest + "-" + first_char_str + "ay";
        }
    }
    
    // Se string vazia, retorna vazia
    string.to_string()
}