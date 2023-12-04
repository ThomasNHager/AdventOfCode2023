use std::fs::File;
use std::io::Read;
use std::any::type_name;
use regex::Regex;
use std::vec;

fn main(){
    // Load the input
    //let mut data = File::open("input.txt").unwrap();
    //let mut content = String::new();
    //data.read_to_string(&mut content).unwrap();
    //let input_vec = content.split("\n");

    let mut toy_data = File::open("toy_input.txt").unwrap();
    let mut toy_content = String::new();
    toy_data.read_to_string(&mut toy_content).unwrap();
    let toy_input_vec = toy_content.split("\n");

    let numeric_list =
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut oldest_line: Vec<String> = Vec::new();
    let mut middle_line:  Vec<String> = Vec::new();
    let mut youngest_line:  Vec<String> = Vec::new();

    for line in toy_input_vec{
        let working_line_tokenized = tokenize_line(line, numeric_list);
        oldest_line = middle_line;
        middle_line = youngest_line;
        youngest_line = working_line_tokenized;

        let mut usable_index_list: Vec<i32> = Vec::new();
        
        // Check to see if older lines are empty
        if middle_line.len() == 0 {
            // Pass
        }
        else if oldest_line.len() == 0 {
            // Character index
            let mut i: i32 = 0;
            for cha in middle_line{
                if cha == "n"{
                    if (middle_line.get(i - 1) == "s" |
                        middle_line.get(i + 1) == "s") |
                        (youngest_line.get(i) == "s" |
                         (youngest_line.get(i - 1) == "s" |
                          youngest_line.get(i + 1) == "s"))
                    {
                        usable_index_list.push(i);
                    }

                }
                i += 1;
            }
        }
        else {
            let mut j = 0;
            for cha2 in middle_line{
                if cha == "n"{
                    if (
                        (middle_line.get(i - 1) == "s" |
                         middle_line.get(i + 1) == "s") |
                        (
                         (youngest_line.get(i) == "s" |
                          (youngest_line.get(i - 1) == "s" |
                          youngest_line.get(i + 1))) |
                         (oldest_line.get(i) == "s" | 
                          (oldest_line.get(i - 1) == "s" |
                           oldest_line.get(i) == "s"))
                         )
                    )
                    {
                        usable_index_list.push(j);
                    }

                }

            }

        }



        
    }
}

fn tokenize_line(line: &str, numeric_list: [char; 10]) -> Vec<String>{
    // Function to take a string and tokenize it
    let mut tokenized_line: Vec<String> = Vec::new();
    for cha in line.to_string().chars(){
        if numeric_list.contains(&cha){
            tokenized_line.push((&"n").to_string())
        }
        else if cha == '.'{
            tokenized_line.push((&".").to_string())
        }
        else{
            tokenized_line.push((&"s").to_string())
        }
        println!("{:?}", cha);
        println!("{:?}", tokenized_line);
    }
    return tokenized_line;
}

