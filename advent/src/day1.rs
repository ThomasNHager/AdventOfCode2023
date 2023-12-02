use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main(){
    // Advent of Code Day 1
    let mut data = File::open("input.txt").unwrap();
    let mut content = String::new();
    data.read_to_string(&mut content).unwrap();
    let parts = content.split("\n");

     //Part 1
    let mut sum = 0;
    for line in parts{
        let char_vec: Vec<char> = line.chars().collect();
        let mut first_char = 0;
        let mut last_char = 0;

        for cha in char_vec{
            if cha.is_numeric(){
                first_char = cha.to_digit(10).unwrap();
                break;
            }
        }

        let mut char_vec_rev: Vec<char> = line.chars().collect();
        char_vec_rev.reverse();

        for cha2 in char_vec_rev{
            if cha2.is_numeric(){
                last_char = cha2.to_digit(10).unwrap();
                break;
            }
        }

        println!("{}", line);
        println!("{}", first_char);
        println!("{}", last_char);

        let concatint = 
            [first_char.to_string(),last_char.to_string()].concat();
        println!("{}", concatint);

        let addint: i32 = concatint.parse().unwrap();
        println!("{}", addint);

        sum = sum + addint;
        println!("{}", sum);

    }

    // Part Two
    let toy_dataset = ["twolnine", "eightwothree", "abcone2threexyz",
                      "xtwone3four", "4nineeightseven2", "zoneight234",
                      "7pqrstsixteen", "three2fourthree", "8czzpmvgmlchnkf"];

    let check_list = ["one", "two", "three", "four", "five", "six", "seven",
                     "eight", "nine"];

    let char_list = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut sum2 = 0;
    for toy in parts{
        let mut num_map = HashMap::new();
        let mut char_map = HashMap::new();
        let mut num_map_right = HashMap::new();
        let mut char_map_right = HashMap::new();

        // Left Search
        for num in check_list{
            match toy.find(num){
                Some(number) => num_map.insert(num, number.to_string()),
                None => num_map.insert(num, "NA".to_string())
            };

            //println!("{}", num);
            //println!("{:?}", num_map.get(num));
        }

        // Left Search
        for cha in char_list{
            match toy.find(cha){
                Some(number2) => char_map.insert(cha, number2.to_string()),
                None => char_map.insert(cha, "NA".to_string())
            };

            //println!("{}", cha);
            //println!("{:?}", char_map.get(cha));
        } 

        // Right Search
        for num in check_list{
            match toy.rfind(num){
                Some(number) => num_map_right.insert(num, number.to_string()),
                None => num_map_right.insert(num, "NA".to_string())
            };

            //println!("{}", num);
            //println!("{:?}", num_map.get(num));
        }

        // Right Search
        for cha in char_list{
            match toy.rfind(cha){
                Some(number2) => char_map_right.insert(cha, number2.to_string()),
                None => char_map_right.insert(cha, "NA".to_string())
            };

            //println!("{}", cha);
            //println!("{:?}", char_map.get(cha));
        } 



        let mut min_val = 10000;
        let mut first_num = "zero";
        let mut max_val = -1;
        let mut last_num = "zero";

        for (key, value) in num_map{
            if value == "NA"{
                // Pass
            }
            else {
                let value_int: i32 = value.parse().unwrap();
                if value_int < min_val{
                    min_val = value_int;
                    first_num = key;
                }
            }
        }

        for (key2, value2) in char_map{
            if value2 == "NA"{
                // Pass
            }
            else {
                let value2_int: i32 = value2.parse().unwrap();
                if value2_int < min_val{
                    min_val = value2_int;
                    first_num = key2;
                }
            }
        }

        for (key3, value3) in num_map_right{
            if value3 == "NA"{
                // Pass
            }
            else {
                let value3_int: i32 = value3.parse().unwrap();
                if value3_int > max_val{
                    max_val = value3_int;
                    last_num = key3;
                }
            }
        }

        for (key4, value4) in char_map_right{
            if value4 == "NA"{
                // Pass
            }
            else {
                let value4_int: i32 = value4.parse().unwrap();
                if value4_int > max_val{
                    max_val = value4_int;
                    last_num = key4;
                }
            }
        }

        let mut first_val = 0;
        let mut last_val = 0;

        let mut i = 1;
        for num2 in check_list{
            if first_num == num2{
                first_val = i;
            }
            if last_num == num2{
                last_val = i;
            }
            i = i + 1;
        }

        let mut j = 1;
        for num3 in char_list{
            if first_num == num3{
                first_val = j;
            }
            if last_num == num3{
                last_val = j;
            }
            j = j + 1;
        }


        println!("{}", "\n");
        println!("{}", toy);
        println!("{}", "Min");
        println!("{}", min_val);
        println!("{}", first_num);
        println!("{}", first_val);

        println!("{}", "Max");
        println!("{}", max_val);
        println!("{}", last_num);
        println!("{}", last_val);

        let tens_digit = first_val * 10;
        let append_num = tens_digit + last_val;
        println!("{}", append_num);

        sum2 = sum2 + append_num;
        println!("{}", "Running Total");
        println!("{}", sum2);

    }



}
