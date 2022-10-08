fn main() {
    // let width1 =30;
    // let height1 =50;
    let rect1 = (30,50);
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1,height1)
         area(rect1)
    );
   // println!("Hello, world!");
    fn area(dimensions:(u32,u32))->u32{
       dimensions.0 * dimensions.1
   }
}
