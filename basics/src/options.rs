pub fn options_demo() {
    let x: Option<i32> = Some(5); // let x = Some(5);
    let y: Option<i32> = None;

    match x {
        Some(v) => println!("x has a value: {}", v),
        None => println!("x is None"),
    }

    match y {
        Some(v) => println!("y has a value: {}", v),
        None => println!("y is None"),
    }


    let v: Result<i32, &str> = Ok(5);
    let r: Result<i32, &str> = Err("error");
    match v {
        Ok(v) => println!("Result is Ok: {}", v),
        Err(e) => println!("Result is Err: {}", e),
    }

}