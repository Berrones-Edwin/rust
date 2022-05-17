// Convertir strings to pig latin. La primera consonante de cada palabra se mueve al final de la palabra y se agrega “ay”, por lo que “first” se convierte en “irst-fay.” Las palabras que comienzan con una vocal tienen “hay”agregado al final ("apple "Se convierte en “apple-hay”). ¡Tenga en cuenta los detalles sobre la codificación UTF-8!

// Utilizando un hash maps y vectores, cree una interfaz de texto para permitir que un usuario agregue nombres de empleados a un departamento de una empresa. Por ejemplo, "Agregar Sally a la ingeniería" o "Agregar Amir a las ventas". Luego, permita que el usuario obtenga una lista de todas las personas de un departamento o de todas las personas de la empresa por departamento, ordenados alfabéticamente.

// use std::{collections::HashMap, vec,io};

use std::io;
mod tree;

use tree::print_gretting;

fn main() {
    // let vowels = ['a', 'e', 'i', 'o', 'u'];
    // println!("Enter a word");

    // let mut word = String::new();

    // match io::stdin().read_line(&mut word) {
    //     Ok(n) => println!("{}", n),
    //     Err(err) => println!("{}", err),
    // }
//    io::stdin().read_line(&mut word).expect("Error read to line");

//     let first_letter = word.to_string().to_lowercase().chars().next().unwrap();

//     let mut flag = false;
//     for i in vowels{
//         if i ==first_letter {
//             flag = true;
//         }
//     }

//     if flag {
//          word.push_str("-hay");
//         println!("{:?}",word.trim());
//     }else{
//         let word_without_first_word = &mut word[1..].to_string();
//         word_without_first_word.push_str("-");
//         word_without_first_word.push_str(&first_letter.to_string());
//         word_without_first_word.push_str("ay");
//         println!("{}",word_without_first_word);
//     }

    print_gretting(String::from("Edwin"));

    

}
