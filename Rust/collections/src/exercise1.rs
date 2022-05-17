// Dada una lista de enteros, usa un vector y regresa la media (el valor promedio), la media (cuando se clasifica, el valor en la posición media) y el modo (el valor que ocurre con mayor frecuencia, un hash maps será útil aquí ) de la lista.

//Promedio => sumar todos y dividirlos por la cantidad total

//Mediana => ordenar de menor a mayor y sacar el numero central
// si hay numero impar toma el del medio
// si hay numero par tomar los del centro y sumarlos

// Moda=> es el numero que más se repite

// Convertir strings to pig latin. La primera consonante de cada palabra se mueve al final de la palabra y se agrega “ay”, por lo que “first” se convierte en “irst-fay.” Las palabras que comienzan con una vocal tienen “hay”agregado al final ("apple "Se convierte en “apple-hay”). ¡Tenga en cuenta los detalles sobre la codificación UTF-8!

// Utilizando un hash maps y vectores, cree una interfaz de texto para permitir que un usuario agregue nombres de empleados a un departamento de una empresa. Por ejemplo, "Agregar Sally a la ingeniería" o "Agregar Amir a las ventas". Luego, permita que el usuario obtenga una lista de todas las personas de un departamento o de todas las personas de la empresa por departamento, ordenados alfabéticamente.

use std::{collections::HashMap, vec};

fn main() {
    let my_numbers = vec![50, 1, 1, 6, 598, 32, 54, 1, 36, 5, 9, 1, 4, 49, 4, 4, 6];

    // let count: i32 = my_numbers.iter().sum();
    // println!("{}", count);

    // my_numbers.sort();
    // println!("{:?}", my_numbers);

    // let mut sum = 0;

    // let left = my_numbers[my_numbers.len() / 2];
    // let right = my_numbers[(my_numbers.len() / 2) - 1];

    // sum = left + right;

    // println!("{}", sum);

    let mut my_hash: HashMap<i32, i32> = HashMap::new();

    let count = 0;
    for i in my_numbers {
        let count = my_hash.entry(i).or_insert(count);
        *count += 1;
    }

    // get item max value
    let result = my_hash.iter().max_by_key(|entry| entry.1).unwrap();

    println!("{:?}", result);
}
