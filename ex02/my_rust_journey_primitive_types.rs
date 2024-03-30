fn main() {
    let uinteger_value_400: u32 = 400;
    let integer_value_14: i32 = 14;
    let uinteger_value_guess: u32 = 0x2f;
    let true_boolean = true;
    let false_boolean = false;
    let float_variable = 3.14;
    let string_variable = "Scotland has 421 words for 'snow'";
    let char_variable = 'z';
    let array_variable = [0, 1, 2, 3, 4];
    let tuple_variable = ("abc", 1, 1.9);
    // let emoji_variable  = '✨';

    println!("Unsigned integer {}", uinteger_value_400 + 2);
    println!("Signed integer = {}", integer_value_14 - 1_000);

    println!("Hex integer = {}", uinteger_value_guess + 0x1f);

    println!("Boolean #1 {}", true_boolean);
    println!("Boolean #2 {}", false_boolean);

    println!("Float -> {}", float_variable);
    println!("String -> {}", string_variable);
    println!("Character -> {}", char_variable);
    println!("Array -> {:?}", array_variable);
    println!("Tuple -> {:?}", tuple_variable);
    // println!("Emoji -> {}", emoji_variable);
}