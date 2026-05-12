fn main() {
    let x=5;
    println!("The value of x is {x}");
    // x=8; this will give error because in rust variable is immutable by default

    // to make it mutable we need to add mut ahead of varialbe name like this let mut x=5;

    
    //Constants - always immutable by default

    const THREE_HOURS_IN_SECONDS:i32=60*60*3;


    //shadowing

    let y=7;
    let y=y+1;
    {
        let y=y*2;
        println!("The value of y in inner scope is: {y}");
    }
    println!("The value of y in outer scope is: {y}");

    // Data types in rust
    // 1: Integer
    // In rust we can have two type of integer first one is unsigned ( only positives ) signed ( + and - values)

    // unsigned integer

    let z:u32=128;

    // signed integer

    let g:i32=-129;


    // 2: Boolean

    let isAdult:bool=true;


    // 3: character

    let c:char='z';


    // 4: Tuple 
    //A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: Once declared, they cannot grow or shrink in size.

    let tup:(i32,f64,u8)=(500,5.6,1);
    let (x,y,z)=tup; // here we destructured elements of tuple just like js

    println!("{x} {y} {z}");

    // 5: Array 
    let months =["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    //You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
    // here as first argument we define its type and sencond one is size of array
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    greet_user(String::from("Raman"));


    //expression and statement in rust
    // in rust expression is something that returns value and statement is just for doing some action without returning anything


    // let i=(let y=8);  this let y =8 is statement it doesnt returns any value so this will throw error
    let j={
        let h=1;
        h+1  // here we havent added semi colon at end unlike we do usually in rust this will become expression so this will return value
    };

    // function with return value
    // type of return value is defined like below
    fn five()->i32{
        5
    };

}


// this is how we define functions in rust 

fn greet_user(name:String){
    println!("hello, {name}");
}
