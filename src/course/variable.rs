// variable types

#[test]
pub fn integer() {
    let int8: i8 = 8; // 8 bit
    let int16: i16 = 16; // 16 bit
    let int32: i32 = 32; // 32 bit
    let int64: i64 = 64; // 64 bit

    let uint8: u8 = 8;
    let uint16: u16 = 16;
    let uint32: u32 = 32;
    let uint64: u64 = 64;

    let intsize: isize = 64; // auto 32/64 bit
    let uintsize: usize = 64; // auto 32/64 bit

    println!("int 8 {}", int8);
    println!("int 16 {}", int16);
    println!("int 32 {}", int32);
    println!("int 64 {}", int64);

    println!("uint 8 {}", uint8);
    println!("uint 16 {}", uint16);
    println!("uint 32 {}", uint32);
    println!("uint 64 {}", uint64);

    println!("intsize {}", intsize);
    println!("uintsize {}", uintsize);
}

#[test]
pub fn float() {
    let float32: f32 = 32.1;
    let float64: f64 = 64.1;

    println!("float 32 {}", float32);
    println!("float 64 {}", float64);
}

#[test]
pub fn string() {
    let string: &str = "Hello World !";

    println!("string : {}", string);
}

#[test]
pub fn boolean() {
    let boolean: bool = false;

    println!("boolean : {}", boolean);
}

#[test]
pub fn char() {
    let char: char = 'a';
    println!("char : {}", char);
}

#[test]
pub fn muttable() {
    // -- muttable variable
    // akan membuat variable baru
    let says: &str;
    says = "its fine..";

    println!("muttable : {}", says);
}

#[test]
pub fn constant() {
    const SHOW_ME: &str = "Hello World";

    println!("constant : {}", SHOW_ME);
}
