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
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1,height1)
        //  area(rect1)
        // Rectangle:: area(&rect1)
        rect1.area()
    );
    println!("打印结构体{:#?}", rect1);
    println!("打印正方体{:?}",Rectangle::square(5));
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
    }
}