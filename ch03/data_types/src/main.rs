fn main() {
    // Tuple Type

    // Create a annotated type tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructure the tuple into three variables
    let (x, y, z) = tup;

    println!("The value of x is {x}");
    println!("The value of y is {y}");
    println!("The value of z is {z}");

    println!("-------------");

    // Now access the values directly by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The index 0 is {five_hundred}");
    println!("The index 1 is {six_point_four}");
    println!("The index 2 is {one}");

    println!("-------------");

    // Array Type

    // Create an array (size fixed, all same type)
    // specifying the type and size
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // Create an array with the same value on all positions
    // Array of size 5 filled with 3s
    let array2 = [3;5];

    // accessing elements of an array
    let first = array[0];
    let second = array[1];

    let first_of_array2 = array2[0];
    let last_of_array2 = array2[4];

    println!("The array2 first item is: {first_of_array2}");
    println!("The array2 last item is: {last_of_array2}");
    println!("The array first item is: {first}");
    println!("The array second item is: {second}");
}
