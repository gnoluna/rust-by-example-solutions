mod display_complex;
mod testcase_list;
mod formatting;
mod tuples;
mod structures;
mod python_to_rust;

fn main() {
    /* 1.2.2 Display
    https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html#activity
    */
    // display_complex::main()

    /* 1.2.2.1 Testcase: List
    https://doc.rust-lang.org/rust-by-example/hello/print/print_display/testcase_list.html#activity

    Activity
    Try changing the program so that the index of each element in the vector is also printed. The new output should look like this:

    [0: 1, 1: 2, 2: 3]
    */
    // testcase_list::main()

    /* 1.2.3 Formatting
    https://doc.rust-lang.org/rust-by-example/hello/print/fmt.html#activity

    Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:


    RGB (128, 255, 90) 0x80FF5A
    RGB (0, 3, 254) 0x0003FE
    RGB (0, 0, 0) 0x000000
    */

    // formatting::main()

    /* 2.2 Tuples
    https://doc.rust-lang.org/rust-by-example/primitives/tuples.html#activity
    Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch 
    from printing the debug format {:?} to the display format {}, you see the following output:
    
    ( 1.1 1.2 )
    ( 2.1 2.2 )

    Add a transpose function using the reverse function as a template, which accepts a matrix as an argument, 
    and returns a matrix in which two elements have been swapped. For example:


    Matrix:
    ( 1.1 1.2 )
    ( 2.1 2.2 )
    Transpose:
    ( 1.1 2.1 )
    ( 1.2 2.2 )

    */
    // tuples::main()

    /* 3.1 Structures
    https://doc.rust-lang.org/rust-by-example/custom_types/structs.html#activity
    1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
    2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top 
    left corner on the point, and a width and height corresponding to the f32.

    */
    // structures::main();

    python_to_rust::main();
}
