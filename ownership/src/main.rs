fn main() {
    // In Rust heap variable like string will always have a single owner and if all of their owner goes out of scope, they get deallocated
    //any time the owner of heap variable goes out of the scope the value is deallocated

    // Example 1
    // passing strings to function as argument
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}",s1) // this will give error because as we discuessed heap variable can only have one owner
     // in above code we are doing let s2=s1;  so here we aer passing owner ship of that s1 variable to s2
    // so after passing ownership you can't then access variable s1 again 

    // Example 2 

    let mut my_string=String::from("hello");
    // take_ownership(my_string);
    // println!("{}",my_string) 
    //same here if we pass string as parameter to function than we no longer own that perticuler strings ownership 
    // that will get passed to that function

    // one fix of above code is to use clone to clone the string 

    take_ownership(my_string.clone());
    // this will create clone of out given string rather passing ownership of referenced variable

    // if we dont want to user clone which is expensive operation tbh than we can return string from 
    // ownership function after using that string and pass back result of it to own main string like below

    my_string=take_ownership_with_return(my_string);
    
}

fn take_ownership(some_string:String){
    println!("{}",some_string);
}

fn take_ownership_with_return(some_string:String)-> String{
    println!("{}",some_string);
    return some_string;
}