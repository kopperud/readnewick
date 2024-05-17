use regex::Regex;

pub fn stripcomments(contents: &str) -> String {
    let re = Regex::new(r"\[.*?\]").unwrap();
    let stripped_contents = re.replace_all(&contents, "");

    return stripped_contents.to_string();
}

pub fn find_newick_string(contents: String) -> String {
    let lparen = contents.find('(').unwrap();

    let semicolon = contents.rfind(';').unwrap();

    let res = contents
        .get(lparen..(semicolon+1))
        .unwrap()
        .to_string();

    return res
}
