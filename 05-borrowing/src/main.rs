fn main() {
    let mut a = vec![1,2,3];
    {
       let b = &mut a; // &mut borrow of `a` starts here
       println!("b = {:?}", b);
    }
    println!("a = {:?}", a); // 打印a的值

    let c = get_fisrt_element(&mut a);
    println!("c = {:?}", c);
    println!("a = {:?}", a); // 再次打印a的值
}

fn get_fisrt_element(g: &mut Vec<i32>) -> i32 {
    g[2] = 9;
    g[0]
}