fn main() {
    println!("Hello, world!");
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn() {
    let a: i32 = 10;
    let b: i32 = 20;
    let c: i32 = a + b;

    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    let s1: String = String::from("hello");
    let s2: String = String::from("world");
    let combined= format!("{} {}",s1,s2);

    println!("Combination of {} and {} is {}", s1, s2, combined);
}

fn update_string(){
    let mut s=String::from("Initial string");
    println!("Before update :{}",s);
    println!("Capacity : {}, Length : {}, Pointer : {:p}",s.capacity(),s.len(),s.as_ptr());

    s.push_str("add some additional text");
    println!("After update :{}",s);
    println!("Capacity : {}. Length : {}, Pointer: {:p}",s.capacity(),s.len(),s.as_ptr());
}
