use std::fs::File;
use std::io::Read;

fn main(){
    // Load the input
    let mut data = File::open("input.txt").unwrap();
    let mut content = String::new();
    data.read_to_string(&mut content).unwrap();
    let input_vec = content.split("\n");

    let mut toy_data = File::open("toy_input.txt").unwrap();
    let mut toy_content = String::new();
    toy_data.read_to_string(&mut toy_content).unwrap();
    let toy_input_vec = toy_content.split("\n");

    let blue_max = 14;
    let green_max = 13;
    let red_max = 12;

    let mut game_sum = 0;
    let mut i = 1;
    let mut cube_sum = 0;
    for line in input_vec{
        let mut game_list = line.split(";");

        let mut most_blue = 0;
        let mut most_green = 0;
        let mut most_red = 0;

        let mut total_blue = 0;
        let mut total_green = 0;
        let mut total_red = 0;

        for  game in game_list{
            println!("{}", game);

            let mut blue_index: String;
            let mut green_index: String;
            let mut red_index: String;

            match game.find("blue"){
                Some(number) => blue_index = number.to_string(),
                None => blue_index = "-1".to_string()
            }

            match game.find("green"){
                Some(number) => green_index = number.to_string(),
                None => green_index = "-1".to_string()
            }

            match game.find("red"){
                Some(number) => red_index = number.to_string(),
                None => red_index = "-1".to_string()
            }

            let mut blue_value_index1: usize = 0;
            let mut green_value_index1: usize = 0;
            let mut red_value_index1: usize = 0;

            let mut blue_value_index2: usize = 0;
            let mut green_value_index2: usize = 0;
            let mut red_value_index2: usize = 0;

            let mut blue_number: u32 = 0;
            let mut green_number: u32 = 0;
            let mut red_number: u32 = 0;

            if blue_index == "-1".to_string(){
                // Pass
            }
            else {
               blue_value_index1 = blue_index.parse().unwrap();
               blue_value_index1 = blue_value_index1 - 3;
               blue_value_index2 = blue_value_index1 + 1;
               if game.chars().nth(blue_value_index1).unwrap() ==
                   " ".chars().nth(0).unwrap(){
                       blue_number =
                           game.chars().nth(blue_value_index2).unwrap().
                           to_digit(10).unwrap();
               }
               else {
                   blue_number = 
                       game.chars().nth(blue_value_index1).unwrap().to_digit(10).
                       unwrap() * 10 + game.chars().nth(blue_value_index2).
                       unwrap().to_digit(10).unwrap();
               }
            }



            if green_index == "-1".to_string(){
                // Pass
            }
            else {
               green_value_index1 = green_index.parse().unwrap();
               green_value_index1 = green_value_index1 - 3;
               green_value_index2 = green_value_index1 + 1;
               if game.chars().nth(green_value_index1).unwrap() ==
                   " ".chars().nth(0).unwrap(){
                       green_number =
                           game.chars().nth(green_value_index2).unwrap().
                           to_digit(10).unwrap();
               }
               else {
                   green_number = 
                       game.chars().nth(green_value_index1).unwrap().
                       to_digit(10).unwrap() * 10 + game.chars().
                       nth(green_value_index2).unwrap().to_digit(10).unwrap();
               }
            }


            if red_index == "-1".to_string(){
                // Pass
            }
            else {
               red_value_index1 = red_index.parse().unwrap();
               red_value_index1 = red_value_index1 - 3;
               red_value_index2 = red_value_index1 + 1;
               if game.chars().nth(red_value_index1).unwrap() ==
                   " ".chars().nth(0).unwrap(){
                       red_number =
                           game.chars().nth(red_value_index2).unwrap().
                           to_digit(10).unwrap();
               }
               else {
                   red_number = 
                       game.chars().nth(red_value_index1).unwrap().to_digit(10).
                       unwrap() * 10 + game.chars().nth(red_value_index2).
                       unwrap().to_digit(10).unwrap();
               }
            }


            if blue_number > most_blue{
                most_blue = blue_number;
            }
            if green_number > most_green{
                most_green = green_number;
            }
            if red_number > most_red{
                most_red = red_number;
            }

        }

        let mut total_cubes = most_blue * most_green * most_red;
        cube_sum += total_cubes;

        if (most_blue <= blue_max) &
            ((most_green <= green_max) & (most_red <= red_max)){
            game_sum = game_sum + i;
        }
        i = i + 1;
        //println!("{:?}", game_sum);
        println!("{}", "Cube Sum");
        println!("{:?}", cube_sum);
    }

}
