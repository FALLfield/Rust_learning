use std::fmt::{Debug, Display};

struct Point<T>{//Use generics defining struct
    x: T,
    y: T,
}
struct Point2<T,U>{//Use multiple generic types defining struct
    x: T,
    y: U,
}
enum Result<T,E>{//To determine correctness
    Ok(T),
    Err(E),
}
impl<T> Point<T>{//Use generics to define method
    fn x(&self) -> &T{
        &self.x
    }
}

pub trait Summary{
    fn summarize(&self) -> String{
        String::from("Read more....")//Default define
    }
}
//trait only define what the behavior looks like, not what the behavior is exactly like
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}
impl Summary for Post {// We can sue Summary for this struct impl since it has this feature
    fn summarize(&self) -> String {
       format!("Article {}, author is {}, ", self.title, self.author)//Override
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
//cmp_display 方法，并不是所有的 Pair<T> 结构体对象都可以拥有，只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法

pub fn notify(item: &impl Summary){//Use trait as arguments
    println!("Breaking news! {}", item.summarize());
    //你可以使用任何实现了 Summary 特征的类型作为该函数的参数，
    // 同时在函数体内，还可以调用该特征的方法，例如 summarize 方法。
}
pub fn notify2<T: Summary>(item: &T, item2: &T){}// Multiple arguments trait restrict
pub fn notify3<T: Summary + Display>(item: &T){}//Multiple implements
pub fn notify4(item: &(impl Summary + Display)){}//语法糖形式

fn some_function<T,U>(t: &T, u: &U) -> u32
    where T: Display + Clone,
          U: Clone + Debug
{1}//Use 'where' to restrict

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T{//Not all the T type can be added, so we need std::ops::Add<Output = T> to restrict
    a + b
}

fn display_array(arr: &[i32]){//Can deliver i32 array
    println!("{:?}",arr);
}
fn display_array1<T: std::fmt::Debug>(arr: &[T]){//Can deliver all types array
    println!("{:?}", arr);
}
fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]){
    println!("{:?}", arr);
}
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Post {
            title: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Weibo {//Compile Error, since we can only return one specified type of Summary
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
        }
    }
}
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}//Fix the largest function, we use PartialOrd and Copy to restrict the return type, so it can be accessed


fn main(){
    let integer = Point{x: 5, y: 5};//We should ensure the type is consist
    println!("Add i8: {}", add(2i8, 3i8));

    //const generics
    let arr: [i32; 3] = [1,2,3];
    display_array(&arr);
    display_array1(&arr);
    display_array2(arr);
    let arr1: [i32; 2] = [1,2];//[i32;2] and [i32;3] are different types
    display_array(&arr1);

    //trait
    //A trait defines a set of behaviors that can be shared, and you can use that set of behaviors as long as the feature is implemented.
    //关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
    //例如我们可以为上面的 Post 类型实现标准库中的 Display 特征，这是因为 Post 类型定义在当前的作用域中。
    // 同时，我们也可以在当前包中为 String 类型实现 Summary 特征，因为 Summary 定义在当前作用域中。
    //在本书中，形如 #[derive(Debug)] 的代码已经出现了很多次，这种是一种特征派生语法，被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。
    //
    // 例如 Debug 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 println!("{:?}", s) 的形式打印该结构体的对象。
    //
    // 再如 Copy 特征，它也有一套自动实现的默认代码，当标记到一个类型上时，可以让这个类型自动实现 Copy 特征，进而可以调用 copy 方法，进行自我复制。

    //特征对象

}