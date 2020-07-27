fn main()
{
    let guess: u32 = "42".parse().expect("Not a number!"); // u32 - 32bit unsigned integer

    let y: f32 = 3.0; // f32 - 32bit floating point

    let f: bool = false; // with explicit type annotation

    let c = 'z'; // 4byte unicode scalar by default

    /*
    Integers
        Length	Signed	Unsigned
        8-bit	i8  	u8
        16-bit	i16 	u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize
    */

    // Compound Types
    let tup = (500, 6.4, 1); // Fixed length grouping of mixed type values into one type - Tuple
    let (a, b, c) = tup; // Destructuring, can use dot

    let array: [i16; 5] = [1, 2, 3, 4, 5]; // Declare array of i16 and length 5
    // let array = [3; 5]; // Create an array of length 5 containing only 3




    /*
    When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. 
    Relying on integer overflow’s wrapping behavior is considered an error.
    If you want to wrap explicitly, you can use the standard library type Wrapping.
    */
    println!("Types - unsignedInt32b: {}, float32b: {}, explicitBool: {}, character: {}. \n", guess, y, f, c);
    println!("Compund Types - Tuple: firstPoint; {}, secondPoint; {}, thirdPoint; {}; struct; ({}, {}, {}), Array: [{}, {}, {}, {}, {}]", a, b, c, tup.0, tup.1, tup.2, array[0], array[1], array[2], array[3], array[4]);
}

