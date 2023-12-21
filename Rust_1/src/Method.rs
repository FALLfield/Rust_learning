struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl Circle {//Methods have the same name with struct
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    //The reason for choosing &self is the same as using &Rectangle in a
    // function: we don't want to take ownership of it, and we don't need to change it, we just want to be able to read the data in the structure.
    pub fn x(&self) -> f64{
        return self.x;//Getter method
    }
}
#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

fn main() {

}
