/*
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) -> i32
{
    *total = 0;
    let mut current = low;
    while current <= high{
        *total += current;
        current += step;
    }
    *total
}   

        

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}
*/



fn most_frequent_word(text: &str) -> (String, usize) {
    
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut FreqWord = String::new();
    let mut highestCount = 0;
    for i in 0..words.len(){
        let mut count = 0;
        for j in 0..words.len(){
            if words[j] == words[i]{
                count += 1;
            }
        }
        if count > highestCount{
            highestCount = count;
            FreqWord = words[i].to_string();
        }
    }
    (FreqWord,highestCount)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
