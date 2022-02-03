mod fundamental;

use fundamental::{
    count_sheep::{
        count_sheep_bool_simple_loop,
        sheep_filter_and_count,
       
    },
};
fn main() {

    println!("Hello, world!");
    println!("Try to test all scripts, but you can run all tests with : cargo test");
    println!("Each script have different ways (functions) to solve the problem. ");

    // Mod sheep bool
    assert_eq!(count_sheep_bool_simple_loop(&[true, false]), 1);
    assert_eq!(count_sheep_bool_simple_loop(&[true,false, true, true, false]), 3);
    assert_eq!(count_sheep_bool_simple_loop(&[true,false, true, true, false, true]), 4);
    assert_eq!(count_sheep_bool_simple_loop(&[true,false, true, true, false, true, true, true]), 6);

    // test
    assert_eq!(sheep_filter_and_count(&[true,false, true, true, false, true, true, true]), 6);

}



