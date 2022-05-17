fn main() {
    // Create new vector
    let mut my_vec: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Add items
    my_vec.push(1);
    my_vec.push(2);

    // print vec
    println!("{:?}", v);
    println!("{:?}", my_vec);

    // get  elements
    //get reference | if index is not defined rust will be panic
    let x = &my_vec[1];
    println!("get element {}", x);

    // Second way
    // Using method get
    // get return option<&T>
    let y = my_vec.get(1);
    println!("method get element {:?}", y);

    match y {
        None =>println!("Error index"),
        Some(_y) => {
            println!("using match {:?}", _y)
        }
    }

    for i in &v {
        println!("{}",i);
    }

    for i in &mut my_vec{
        *i += 50;
    }

    println!("{:?}", my_vec);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(6.3),
        SpreadsheetCell::Text(String::from("Lorem Ipsum"))
    ];

    println!("{:?}",row);
}
