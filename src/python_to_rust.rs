pub fn main() {
    /* Variables
        1. Assign values
        Python: 
            a = 1
            b = '10'
            c, d = True, False 
    */

    // Rust
    let a = 1;
    let b = "10";
    let (c, d) = (true, false);
    
    /* Printing styles
        Python:
            print(a, "\n")
            repr(b)
            print(f"{c}, {d}")
    */

    // Rust
    println!("{}", a);
    print!("{:?}", b);
    eprint!("{}", c);
    print!("{}, {}", c, d);
 
    /*
        Data Types

        String, Char

        Python:
            a: str = "string"
            b: str = "c"
    */

    // Rust
    let _a: &str = "string"; // explicit
    let _c: char = 'c';

    /*
        Array (List)
    */

    // Rust

    let l: [i32; 5] = [1,2,3,4,5]; // l = [1,2,3,4,5]
    l.len(); //len(l) 
    l.first(); // l[0]
    l.last(); // l[-1]
    let _ = &l[1..2]; // print(l[1:2]) list slicing
    let _ = &l[..]; // l[:]
    l.is_empty(); // len(l) == 0
    ["a", "b", "c"].join(","); // ','.join("abc")


    // l[::2]
    let mut iter = l.chunks(2);
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.next().unwrap()); 
    println!("{:?}", iter.next().unwrap()); 

    // "abc".split('b')
    let mut iter = "abc".split(|num| num == 'b'); 
    println!("{}", iter.next().unwrap());
    println!("{}", iter.next().unwrap());

    // referring slice
    let _slice: &[i32] = &l[2..4]; // no need to specify length for slices
    let _slice: &[i32] = &l[..]; // mutable reference


    // Tuples

    let person_data = ("Alex", 48, "35kg", "6ft");
    println!("{}", person_data.0); // person_data[0]


    /*
        Logical operators
    */

    let (a, b) = (true, false);
    println!("{}", a && b); // a and b
    println!("{}", a || b); // a or b
    println!("{}", !a); // not a



    /*
        Conditional Expression
    */


    /*
        For loop

        Python:
            for i in range(10):
                print(i)
    */

    for i in 0..10 { 
        print!("{}", i);        
    }

    /*
        Iterators (List comprehensions)

        Python:
            print([x*2 for x in range(10) if x % 2 == 0])
    */

    let v = (0..10).filter(|x| x%2 == 0).map(|x| x*2).collect::<Vec<i32>>();
    println!("{:?}",v);


}