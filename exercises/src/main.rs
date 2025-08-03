mod vector;

use vector::median_and_mode;

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

    println!("==== Exercise 2: ... ====");

    {
        
    }
}
