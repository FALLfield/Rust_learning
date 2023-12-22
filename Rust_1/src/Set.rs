use std::collections::HashMap;

#[derive(Debug)]
enum Add{
    Ipv4(i32),
    Ipv6(String),
}

trait IpAddr{
    fn display(&self);
}
struct v4(String);
impl IpAddr for v4 {
    fn display(&self) {
        todo!()
    }
}
struct v6(i32);

impl IpAddr for v6 {
    fn display(&self) {
        todo!()
    }
}


fn main(){
    //Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let v2 = vec![1,2,3];
    //Vector 在超出作用域后会被删除
    let third = &v[1];
    match v.get(2){
        Some(Third) =>println!("The third element is {}",Third ),
        None => println!("No matching number"),//.get() will ensure there won't be error, if the index out of range, it will return None
    }
    for i in &v{
        println!("{}", i);
    }

    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);//Float sort

    let mut v5 = Vec::with_capacity(10);
    v5.extend([1, 2, 3]);    // 附加数据到 v5

    //Store different types
    let mut v3 = vec![
        Add::Ipv6(String::from("aaabbbb")),
        Add::Ipv4(111222),
    ];//Enum implement
    let mut v4 = vec![
        Box::new(v4("aaabbb".to_string())),
        Box::new(v6(123)),
    ];//Trait feature implement

    //HashMap
    let mut my_gems = HashMap::new();
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();//Transfer vec to HashMap

    let name = String::from("Sunface");
    let age = 18;

    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);

    std::mem::drop(name);
    println!("因为过于无耻，{:?}已经被除名", handsome_boys);
    println!("还有，他的真实年龄远远不止{}岁", age);
    //如果你使用引用类型放入 HashMap 中，请确保该引用的生命周期至少跟 HashMap 活得一样久

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);//get() return an Option<i32>, we use borrow to avoid ownership transfer
    // 覆盖已有的值
    let old = scores.insert(String::from("Blue"), 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry(String::from("Yellow")).or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry(String::from("Yellow")).or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入




}