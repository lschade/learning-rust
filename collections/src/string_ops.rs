pub fn pig_lating(s: &String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut words = Vec::new();

    for w in s.split_whitespace() {
        let first_char = w.chars().nth(0).expect("empty");
        if vowels.contains(&first_char) {
            words.push(format!("{}-hay", w));
        } else {
            let mut new_w = String::from(w);
            new_w.replace_range(..1, "");
            words.push(format!("{}-{}ay", new_w, w.chars().nth(0).unwrap()));
        }
    }

    return words.join(" ");
}