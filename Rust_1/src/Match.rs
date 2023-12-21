enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum IpAddr {
    Ipv4,
    Ipv6,
}

fn main() {
    //match & if let
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    }; //Match can return a value

    println!("{}", ip_str);

    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    //We can use if let to shorten the statements
    if let Some(3) = v {
        println!("Three");
    }

    let v1 = vec![Coin::Dime, Coin::Nickel];
    v1.iter().filter(|x| matches!(x, myEnum::Foo));
    //variable hidden
    let age = Some(20);
    println!("Before matching, age is {:?}", age); //Some(30)
    if let Some(age) = age {
        println!("In the matching, age is {:?}", age); //30
    }
    println!("After matching, age is {:?}", age); //Some(30)


    //Option
    let five = Some(5);
    let six = plus_one(five);//return Some(6)
    let none = plus_one(None);//return None

    //deconstruct
    struct Point{
        x: i32,
        y: i32,
    }
    let p = Point {x : 0, y : 7};
    let Point{x: a, y: b} = p;
    assert_eq!(0, a);

    //Bund @
    enum Message{
        Hello {id: i32},
    }
    let msg = Message::Hello {id: 5};
    match msg {
        Message::Hello {id: id_variable @ 3..=7} =>{
            println!("Found an id in range: {}", id_variable)// Bunbled the value to a variable
        },
        _ = > {
            println!("Found some other id: {}", id)
        },
    }

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}: ", state);
            25
        } //Take the value stored in enum
    }
}

//Option
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
