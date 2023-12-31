#![allow(unused_variables)]//引入 #![allow(unused_variables)] 属性标记，该标记会告诉编译器忽略未使用的变量
type File = String;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
#[derive(Debug)]
enum PokerSuit{
    Clubs(u8),
    Spades(u8),//We can point the value type of enum
    Diamonds,
    Hearts,
}

#[derive(Debug)]
struct PokerCard{
    suit: PokerSuit,
    value: u8,
}
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()//unimplemented!() 标记通常意味着我们期望快速完成主要代码，回头再通过搜索这些标记来完成次要代码
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(&mut f1, &mut vec![]);
    close(&mut f1);

    let mut s = String::from("Hello world!");
    let refs = &s[..];//Reference to get String slice
    let hello = &s[0..5];//String slice
    let world = "world";//String slice
    //在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置
    let zhongwen = "中国人";
    let zhongwenslice = &[0..2];//这样写是错误的，因为每个中文占了三个字节

    let s1 = s.push_str("a");//追加
    let s2 = s.insert(5, "a");//插入字符
    let s3 = s.replace("Hello", "hhhh");//replace
    let c1 = s.pop();//Delete the last character and return it
    let c2 = s.remove(3);//Delete the specified index value and return it


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);//切片同样适用于数组


    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    //Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let index = tup.0;//Use "." to access tuple element


    //Struct
    let user1 = User {
        email: String::from("example"),
        username: String::from("aa"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another"),
        ..user1
    };//Get left parts from user1 and only modify the email
    //这是结构图更新语法，部分值的所有权会转移到user2身上

    //Tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    //Derive display struct
    let ret1 = Rectangle {
    width: 30,
    height: 50,
    };
    println!("ret1 is {:?}", ret1);
    dbg!(&ret1);//The dbg! will take the ownership of ret1 and return it finally



    // Enumerate
    let heart = PokerSuit::Hearts;
    print_suit(heart);

    let c1 = PokerCard{
        suit: PokerSuit::Clubs,
        value: 1,
    };

    let c2 = PokerSuit::Spades(5);

    let some_number = Some(5);
    let some_string = Some("A string");

    let absent_number: Option<i32> = None;// If we use None, we need Rust what type the Option<T> is

    //Array
    let a = [1,2,3,4,5];//Allocate an static array
    let a = [3; 5];//Allocate an array that has 5 3s
    let first = a[0];//Access a's index 0
    let array: [String; 8] = std::array::from_fn(|_i| String::from("Rust is good! "));//Use this statement to allocate an array that String repeat 8 times




}

fn print_suit(card: PokerSuit){
    println!("{:?}", card);
}
fn calculate_length(s: String) -> (String,usize){
    let length = s.len();
    (s, length)
}//Use tuple return two values

