#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8
}

// 单元结构体
#[derive(Debug)]
pub struct Unit;

// 元组结构体
#[derive(Debug)]
pub struct Pair(pub i32, pub f32);

pub struct Point {
    x: f32,
    y: f32,
}