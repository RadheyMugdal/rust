fn main() {
    let number =3;
    if number < 5 {
        println!("condition was true");
    }else{
        println!("conditioon was false");
    }

    // if we do it like below that put this condition that if number {} than rust will give error unlike othe language that we use like js that autometically converts it to boolean value 

    // if number{
    //     println!("number was three");
    // }

    // else if 
    

    if number %4==0{
        println!("number is divisible by 4");
    }else if number % 3==0{
        println!("number is divisible by 3");
    }else{
        println!("number is not divisible by 4.3 or 2");
    }

    // using if in a let statement 
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    
    let condition=true;
    let number=if condition { 5 } else {6};
    println!("The value of number is: {number}");
    

    // Loops in rust

    // loop keyword tells rust to execute a block of code over and over again either forever or until you explicitely tell it to stop.
    // we can come out of loop using break keyword and it also provides continue keyword which will skip rest of code in current loop, break breaks loop
    
    let mut counter=0;

    loop {
        if counter==10{
            break;
        }

        counter+=1;
        println!("Again!")
    }


    // Disambiguating with Loop Labels

    /* If you have loops within loops, break and continue apply to the innermost loop at that point. 
    You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
    Loop labels must begin with a single quote. Here’s an example with two nested loops:
     */

    let mut count=0;

    'counting_up:loop{
        println!("count = {count}");
        let mut remaining =10;
        
        loop{
            println!("remaining = {remaining}");
            if remaining==9{
                break;
            }

            if count ==2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count+=1;
    }
    println!("End count = {count}");


    let mut num=5;

    while num!=0{
        println!("{num}");
        num-=1;
    }
    println!("LIFTOFF!!!");

    // Looping through collection with while and for loop 

    let a=[10,20,30,40,50];
    let mut index=0;

    while index <5 {
        println!("The value is: {}",a[index]);
        index+=1;
    }

    // For loop

    for element in a{
        println!("The value is : {element}");
    }



}
