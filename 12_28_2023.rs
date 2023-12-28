fn main() {
    // 1
    println!("Hello, world!");
    // 2
    let ans = 50;
    assert_eq!(ans,50);
    
    // 3
    let mut sum = 0;
    for i in 0..5 {
        if i%2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
        sum += i;
    }
    println!("Sum is {}", sum);
    
    // 4
    for i in 10..15 {
        let even_odd = if i%2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
    
    // 5
    let res = sqr(2.0);
    println!("Square result is {}", res);
    
}

// 5
fn sqr(x: f64)-> f64 {x*x}
