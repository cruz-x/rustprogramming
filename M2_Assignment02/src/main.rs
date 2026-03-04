
fn wordfinder(sentence: &str) -> (String, usize) {

    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut max_word = String::new();
    let mut max_count = 0;

    for &word in &words {

        let mut count = 0;

        for &comparedword in &words {

            if word == comparedword {
                count += 1;
            }

        }

        if count > max_count {

            max_count = count;
            max_word = word.to_string(); 

        }

   
    }

    (max_word, max_count)
}

fn main() {
    let sentence = "for the and for the the the me red blue blue red and the for for";
    let (word, amount) = wordfinder(sentence);
    println!("The most used word in the sentence was: '{}', with it being used {} times", word, amount);
}