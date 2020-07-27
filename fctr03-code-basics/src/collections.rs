use std::collections::HashMap;

pub fn run() {
    // Array
    let mut arr = [1, 2, 3];
    arr[0] = 0;

    // Vectors
    let mut vec = vec![1, 2];
    vec.push(3);
    vec[0] = 0;

    // Conversion
    let _arr_from_vec = &vec[0..2];
    let _vec_from_arr = arr.to_vec();

    // Hashmap
    let mut map = HashMap::new();
    map.insert("key", 123);

    println!("[collections]");
    println!("arr: {:#?}", arr);
    println!("vec: {:#?}", vec);
    println!("map: {:#?}", map);
}
