enum Direction{
    East,
    West,
    North,
    South,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum IpAddr{
    Ipv4,
    Ipv6,
}

fn main(){
   //match & if let
    let dire = Direction::South;
    match dire{
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };

    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };//Match can return a value

    println!("{}", ip_str);

    let v = Some(3u8);
    match v{
        Some(3) => println!("three"),
        _ => (),
    }
    //We can use if let to shorten the statements
    if let Some(3) = v{
        println!("Three");
    }

    let v1 = vec![Coin::Dime, Coin::Nickel];
    v1.iter().filter(|x| matches!(x, myEnum::Foo));



}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime =>10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}: ", state);
            25
        },//Take the value stored in enum
    }
}