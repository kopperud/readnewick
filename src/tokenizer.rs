use std::collections::HashSet;


pub fn tokenize(s: &str) -> Vec<String> {
    //let tokens: Vec<str> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();

    //let mut special_tokens = HashSet::new();
    let mut special_tokens = Vec::new();
    special_tokens.push('(');
    special_tokens.push(')');
    special_tokens.push(';');
    special_tokens.push(',');


    let chars = s.chars();
    let mut iter = chars.peekable();
    let mut token = "".to_string();


    while let Some(_) = iter.peek(){
        if let Some(c) = iter.next(){

            //println!("{:?}", &c);
            if c != 0xA as char {

                let is_special = special_tokens.contains(&c);
                token.push(c);

                if is_special{
                    tokens.push(token);
                    token = "".to_string();
                }else{
                    let next = iter.peek().expect("should have been able to peek to next char");
                    if special_tokens.contains(next){
                        tokens.push(token);
                        token = "".to_string();
                    }
                }
            }
        }
    }

    return tokens;
}
