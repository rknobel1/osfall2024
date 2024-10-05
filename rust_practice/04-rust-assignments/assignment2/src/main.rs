fn most_frequent_word(text: &str) -> (String, usize) {

    let mut words = text.split_whitespace().collect::<Vec<_>>();
    // println!("{:?}", words);

    let mut max_word = "".to_string();
    let mut max_count = 0;

    for i in 0..words.len() {
        let mut count = 0;

        let word = words[i];
        if word == "" {continue;}

        count += 1;

        for j in i+1..words.len() {
            if word == words[j] {
                count += 1;
                words[j] = "";
            } 
        }

        if count > max_count {
            max_count = count;
            max_word = word.to_string();
        }
    }

    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}