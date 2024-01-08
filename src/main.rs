fn main() {
    let mut min_len: i32 = 2; // Maxum value that can be assaigned is 14 and minimum is 0
    if min_len > 14 {
        min_len = 14;
    } else if min_len < 0{
        min_len = 0;
    }

    let mut max_len: i32 = 14; // Maxum value that can be assaigned is 14 and minimum is 0
    if max_len > 14 {
        max_len = 14;
    } else if max_len < 0{
        max_len = 0;
    }


    let current_string: String = String::new();
    let word_count: i32 = 100;
    let word_array: [[u8;14]; 100] = initialize_new_words(min_len, max_len, word_count);

    println!("{:?}", word_array[0]);
}

fn initialize_new_words(words_min_len: i32, words_max_len: i32, word_count: i32) -> [[u8;14]; 100] {
    let current_string: String = String::new();
    let bytes:&[u8] = current_string.as_bytes();
    let temp_array: [[u8;14]; 100] = [[0;14]; 100];

    for elements in temp_array {
        for element in elements {

        }
    }
    temp_array
}
