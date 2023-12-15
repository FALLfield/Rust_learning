// In Rust, every value is owned by a variable, which is called owner
//A value can only be owned by one variable
//When owner exit the scope, the value will be dropped
fn main() {
    println!("Hello, world!");

    let s = "hello";//字符串字面值, immutable
    //Mostly, we use String::from() based on string slice
    let s = String::from("hello");
    //String type is stored on heap
    s.push_str(", world");
    println!("{}", s);

    let x = 5;
    let y = x;//Auto copy, since i32 is a basic type

    let s1 = String::from("Hello");
    //This operation is called move
    let s2 = s1;// This points that s1 is not validate, s1 ownership is transferred to s2
    //But Rust will forbidden invalidate reference, so there will throw an error
    let s3 = s1.clone();//Deep copy

    //Some types are copy
    //All the integer type, boolean, float, char, tuple, immutable reference&T

    //When we pass the value to function, there will be a transfer either

    let a = 5;
    let y = &a;
    assert_eq!(5,*y);//y is a reference of x, *y solve the reference

    let len = calculate_length(s1);//We can see that we don't need to transfer out the owner ship

    let mut str = String::from("Hello");
    change(&mut str);//Mutable reference, but in one scope, there can only be one mutable reference for one variable
    //Mutable reference and immutable reference can not exist at the same time
}

fn calculate_length(s: &String) ->usize{
    s.len();
}

fn change(some_string: &mut String){
    some_string.push_str("hhhh");
}

fn dangle() -> &String{
    let s = String::from("Hello");
    &s//悬垂引用，指针指向某个值后这个值却被释放掉了，而指针仍然存在。
}
