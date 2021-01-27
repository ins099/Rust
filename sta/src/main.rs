use std::mem;
fn main() {
//                                         STRING

    let s = "STRING";
    println!("{}", s);

    let ss = String::from("Hi this is string from String Function.");
    println!("{}", ss);
        //Since String is not a definite type in RUST it is Compound type, usuallay a tuple or Array it can be sliced!
    
    let slice = &ss[1..4];
    println!("{}",slice);

        // concatenate
    
        let h = ss + &s;
        println!("{}", h);



//                                         TUPLES

    let t1 = ('a',1,4.6);

    //this is how you iterate individual element 
    println!("This is first element {}", t1.0);

    // print all elements in tuple
    println! ("{:?}", t1);

    // tuple can be inside tuple
    let t2 = (1,2,t1);
    println!("{:?}",t2 );
    println!("{:?}",t2.2 );

//                                         ARRAYS

    let xs: [i16; 5] = [1,2,3,4,5];
    // print all values
    println!("{:?}" , xs);
    // indexing 
    println!("{} {} This is lenght {} this is size in memory{}",xs[0], xs[2], xs.len(), 
        mem::size_of_val(&xs)); // mem::size_of_val takes a argument of array and a "&" as referene

    
        // slicing array

        let sl = &xs[2..4];
        println!("this is slice array: {:?}",sl);

        let sl = &xs[0..3];
        println!("this is slice array: {:?}",sl);



}
