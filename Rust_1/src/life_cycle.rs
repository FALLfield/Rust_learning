//生命周期的主要作用是避免悬垂引用，它会导致程序引用了本不该引用的数据
//在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。
struct ImportantExerpt<'a> {
    part: &'a str,//Immutable life noted reference
}
impl<'a> ImportantExerpt<'a>{
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
        where
            'a: 'b,//进行约束，表明‘b 比 ’a活得短
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main(){
    {
        let r;         // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;  // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+

    let novel = String::from("Call me Ishmael");
    let first_sentence = novel.split('.').next().expect("Could not find a .");
    let i = ImportantExerpt{
        part: first_sentence,
    };

}
fn useless<'a>(first: &'a i32, second: &'a i32) {}//这两个参数 first 和 second 至少活得和'a 一样久

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}