#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn main() {
    // let width1 =30;
    // let height1 =50;
    // let rect1 = (30,50);
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle{width:10,height:30};
    let rect3 =Rectangle{width:35,height:55};
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1,height1)
        //  area(rect1)
        // Rectangle:: area(&rect1)
        rect1.area()
    );
    println!("打印结构体{:#?}", rect1);
    println!("打印正方体{:?}",Rectangle::square(5));
    println!("1比2 {}",rect1.duibi(&rect2));
    println!("1比3 {}",rect1.duibi(&rect3));
    // println!("Hello, world!");
    //  fn area(dimensions:(u32,u32))->u32{
    //     dimensions.0 * dimensions.1
// }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }
        fn square(x:u32)-> Rectangle {
            Rectangle{
                width:x,
                height:x,
            }

        }
        fn duibi(&self,other:&Rectangle) -> bool{
            self.width > other.width && self.height> other.height

        }
    }
}