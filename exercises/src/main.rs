mod vector;
mod pig_latin;
mod employees;

use vector::median_and_mode;
use pig_latin::convert_words;
use employees::cli_interface;

fn main() {

    println!("==== Exercise 1: Vectors mode and median ====");
    
    {
        println!("\n Test 1");
        let vec1 = Vec::from([1,2,3,4]);
        println!("{vec1:?}");
        median_and_mode(vec1);

        println!("\n Test2");
        let vec2 = Vec::from([3, 4, 4, 5]);
        println!("{vec2:?}");
        median_and_mode(vec2);

        println!("\n Test3");
        let vec3 = Vec::from([3, 5, 3]);
        println!("{vec3:?}");
        median_and_mode(vec3);

        println!("\n Test4");
        let vec4: Vec<i32> = Vec::from([]);
        println!("{vec4:?}");
        median_and_mode(vec4);
    }

    println!("==== Exercise 2: Pig Latin ====");

    {
        println!("\n Test 1");
        let str1 = String::from("Oie");
        let str1_converted = convert_words(&str1);
        println!("Converted {str1} to {str1_converted}");

        println!("\n Test 2");
        let str2 = String::from("Salve");
        let str2_converted = convert_words(&str2);
        println!("Converted {str2} to {str2_converted}");
    }

    println!("==== Exercise 3: ... ====");

    {
        cli_interface();
    }
}
