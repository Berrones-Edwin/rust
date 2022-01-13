
fn main() {
    // NOTAS
    // todas las variables son inmutables
    // si queremos hacerlas mutables basta con agregar mut
    
    // let x = 5;
    // println!("The value of x is",x);
    // x=6;
    // println!("The value of x is",x);

    // Haciendo mutable una variable
    // let mut x = 5;
    // println!("The value of x is: {}",x);
    // x=6;
    // println!("The value of x is: {}",x);


    //DIFERENCIAS ENTRE CONST Y LET
    // Una constante se declara con const y su valor nunca puede ser mutable
    // Las constantes pueden ser declaradas en cualquier scope, incluso en global scope

    // SHADOWING
    // Podemos declarar una nueva variable usando el mismo nombre, a esto se le conoce como shadowed, el programa vera la segunda variable y tomara el valor de su ultima declaracion

    let x = 5; // Aqui la variable vale 5
    let x = x + 1; // Aqui toma el valor de 6

    {
        let x = x *2; // Aquí toma el valor de 12

        println!("The value of x in th inner scope is {}",x); // result is 12
    }
    println!("The value of x  is {}",x); //result  is 6

    //     El sombreado es diferente de marcar una variable como mut, porque obtendremos un error en tiempo de compilación si intentamos reasignar accidentalmente a esta variable sin usar la palabra clave let. Al usar let, podemos realizar algunas transformaciones en un valor pero hacer que la variable sea inmutable después de que se hayan completado esas transformaciones.

    // La otra diferencia entre mut y shadowing es que debido a que estamos creando efectivamente una nueva variable cuando usamos la palabra clave let nuevamente, podemos cambiar el tipo de valor pero reutilizar el mismo nombre. Por ejemplo, digamos que nuestro programa le pide a un usuario que muestre cuántos espacios quiere entre un texto ingresando caracteres de espacio, pero realmente queremos almacenar esa entrada como un número:

        let spaces = "   ";
        let spaces = spaces.len(); //esta cambiando el tipo de dato string to number


    

}
