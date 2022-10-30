// 面向对象设计 -> 多态 -> 泛型

pub trait Animal {
    fn print(&self);
    fn get_name(&self){ //默认方法实现
        print!("动物-")
    }
}

pub struct User {
    pub components: Vec<Box<dyn Animal>>,
}

impl User {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.get_name();
            component.print();
        }
    }
}

struct Cat {
    pub name: String,
}

impl Animal for Cat {
    fn print(&self) {
        println!("动物名字: {:?}", self.name)
    }
}

pub struct Dog {
    pub name: String,
}

impl Animal for Dog {
    fn print(&self) {
        println!("动物名字: {:?}", self.name)
    }
}


/// 抽象多态的设计
pub fn test1() {
    let user = User {
        components: vec![
            Box::new(Cat { name: "猫".to_string() }),
            Box::new(Dog { name: "狗".to_string() }),
        ],
    };
    user.run();
}