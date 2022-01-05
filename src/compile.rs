// return true if the word is in the dictionary
pub fn is_in_compiled_words(word: String, compiled_words: &Vec<Vec<String>>) -> bool {
    for i in 0..compiled_words.len() {
        if word == compiled_words[i][0] {
            return true;
        }
    }
    return false;
}

// return the  word in the compiled words f existing
pub fn get_word(word: String, compiled_words: &Vec<Vec<String>>) -> Vec<String> {
    for i in 0..compiled_words.len() {
        if word == compiled_words[i][0] {
            return compiled_words[i].clone();
        }
    }
    return Vec::new();
}

// return the index of the word in the compiled words
pub fn get_index(word: String, compiled_words: &Vec<Vec<String>>) -> i32 {
    for i in 0..compiled_words.len() {
        if word == compiled_words[i][0] {
            return i as i32;
        }
    }
    return -1;
}
