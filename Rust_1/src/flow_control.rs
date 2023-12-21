fn main(){
    let condition = true;
    let number = if condition{
        5
    }else if !condition{
        6
    };//We should ensure that every branches has the same value type

    //for loop
    for i in 1..=5{
        println!("{}", i);//Output a 1 to 5 sequence
    }
    let a = [4,3,2,1];
    for(i , v) in a.iter().enumerate(){
        println!("The {} element is {}" , i + 1, v);//我们通常来说更喜欢使用这种方式
    }
    for _ in 0..10{

    }
    //while loop
    while(condition){

    }
    //non condition loop
    loop{

    }

}
