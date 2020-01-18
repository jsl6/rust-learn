fn main() {
    let a = [1, 2, 3];
    let b = a;
    println!("a = {:?}, b = {:?}",a, b);  // 这是正常： a = [1, 2, 3], b = [1, 2, 3]

    let c = vec![1, 2, 3];
    let d = c;

    println!("c = {:?}, d = {:?}",c, d);  // Error: value borrowed here after move
}
