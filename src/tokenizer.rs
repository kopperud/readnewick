
pub fn tokenize(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();

    const SPECIAL_TOKENS: [char; 4] = ['(', ')', ',', ';'];
    let mut start = 0;

    let mut iter = s.char_indices().peekable();

    while let Some((end, c)) = iter.next(){
        if SPECIAL_TOKENS.contains(&c){
            if start != end {
                tokens.push(&s[start..end]);
            }
            tokens.push(&s[end..end + c.len_utf8()]);
            start = end + c.len_utf8();
        }else if c == 0xA as char{
            if start != end {
                tokens.push(&s[start..end]);
            }
            start = end + 1;
        }
    }

    if start < s.len(){
        tokens.push(&s[start..s.len()]);
    }

    return tokens;
}
