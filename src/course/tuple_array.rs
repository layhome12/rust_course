// -- tuple array

#[test]
pub fn tuple() {
    let tuple: (i32, &str) = (32, "hello");

    println!("tuple : {:?}", tuple);
}

#[test]
pub fn array() {
    let array: [i8; 5] = [1, 2, 3, 4, 5];
    let multi: [[i8; 2]; 2] = [[1, 2], [3, 4]];

    println!("array : {:?}", array);
    println!("multi : {:?}", multi);
}

#[test]
pub fn array_tuple() {
    let arr_tuple: [(i32, &str); 2] = [(1, "Hello"), (2, "World")];

    println!("arr_tuple : {:?}", arr_tuple);
}
