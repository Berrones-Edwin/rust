use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let scores_vec = vec![50, 100];

    let scores_2: HashMap<_, _> = teams.iter().zip(scores_vec.iter()).collect();

    println!("{:?}", scores_2);
    println!("{:?}", teams);
    println!("{:?}", scores_vec);

    let field_name = String::from("Blue");
    let field_score = 20;

    let mut map = HashMap::new();

    map.insert(field_name, field_score);

    println!("{} ", field_score);

    //---------------
    //Get values
    //--------------

    let get_some_value = String::from("Blue");

    let my_value = map.get(&get_some_value);

    println!("{:?}", my_value);

    //---------------
    //Iteration values
    //--------------

    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    //---------------
    //overwrite value
    //--------------

    scores.insert(String::from("Blue"), 999);
    println!("{:?}", scores);

    //---------------
    //insert value if it haven't value
    //--------------

    let mut my_new_map = HashMap::new();

    my_new_map.insert(String::from("Yellow"), 500);

    my_new_map.entry(String::from("Blue")).or_insert(200);

    println!("{:?}", my_new_map);

    //---------------
    // update value based in prev value
    //---------------

    let text = "hello world wonderful world world";

    let mut text_hash_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_hash_map.entry(word).or_insert(0);
        println!("before {}", count);
        *count += 1;
        // *count += 1;
        println!("after {}", count);
    }
    println!("after {:?}", text_hash_map);
}
