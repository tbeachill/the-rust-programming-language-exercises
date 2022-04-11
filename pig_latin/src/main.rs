fn main() {
    let test = "this is a test string";

    if let Some(latin) = pig_latin(&test) {
        println!("{}", latin);
    }
}

fn pig_latin(input: &str) -> Option<String> {
    if !input.is_ascii() {
        return None;
    }

    // initialise mutable string to build on
    let mut new_string = String::from("");

    // split string on spaces
    for word in input.split(" ") {
        // append "-hay" to words starting with vowel
        // remove first letter on starting with [c]onsonant and append "-[c]ay"
        if word[0..1].contains(['a', 'e', 'i', 'o', 'u']) {
            new_string += &format!("{}-{}", &word, "hay");
        } else {
            let suffix = word[0..1].to_owned() + "ay";
            new_string += &format!("{}-{}", &word[1..], &suffix);
        }
        new_string += " ";
    }

    Some(new_string.trim().to_string())
}
