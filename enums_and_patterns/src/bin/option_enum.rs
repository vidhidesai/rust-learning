fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);   //unwrap_or(default) method used
                                    //to convert from type Option<T> to T

    println!("Sum is {sum}");
}
