fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    return (boolean, integer);
}
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple with a bunch of differenc types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("long tuple first value {}", long_tuple.0);
    println!("long tuple second value {}", long_tuple.1);
    println!("tuple repr: {:?}", long_tuple);

    // Values can be tuple members
    let tuple_of_tulpes = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuples of tuples {:?}", tuple_of_tulpes);

    /*
    But long Tuples cannot be printed
    ADD 13 to too long tuple to see compiler error
    */
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("{:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}, {:?}", a, b, c, d, d);
    println!("{:#?}", tuple);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{:#?}", matrix);
}
