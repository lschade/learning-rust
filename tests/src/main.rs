fn remove_hidden_lines(input: &str) -> String {
    let mut resulting_lines = vec![];
    let mut within_codeblock = false;

    for line in input.lines() {
        if line.starts_with("```") {
            within_codeblock = !within_codeblock;
        }

        if !within_codeblock || (!line.starts_with("# ") && line != "#") {
            resulting_lines.push(line)
        }
    }

    resulting_lines.join("\n")
}

fn main() {
    let lst = "#[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    }";

    let result = remove_hidden_lines(lst);
    println!("{}", result);
}