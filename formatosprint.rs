fn main(){
    //format! escribe texto a string
    //print! es lo mismo que format! pero lo imprime en la consola
    //println! es lo mismo que print! pero agrega un saltp de linea
    //eprint! lo mismo que print! pero imprime un error estandar
    //eprintln! lo mismo que println! pero imprime un error estandar
    
    //{} es automaticamnte reemplazado por cualquier argumento.
    println!("{} dias", 31);

    //se pueden usar argumentos posicionales, especificando un enteto dentro de {}
    println!("{0}, esto es {1}. {1}, esto es {0}", "Samuel", "Araceli");

    //se pueden usar argumentos nombrados
    println!("{sujeto} {verbo} {objeto}",
        objeto = "the lazy dog",
        sujeto = "the quick brown fox",
        verbo = "jumps over");

    // se pueden invocar difernetes formatos
    println!("Base 10:              {}", 69420); //69420
    println!("Base 2(Binario):      {:b}", 69420); //10000111100101100
    println!("Base 8 (Octal):       {:o}", 69420); //207434
    println!("Base 16 (Hexadecimal):{:x}", 69420); //10f2c

    // se pueden justificar textos con espacios
    println!("{number:>5}", number = 1);

    // se pueden justificar textos con ceros extra a la izquierda
    println!("{number:0>5}", number = 1); //00001
    // tambien se pueden justificar ceros extra a la derecha
    println!("{number:0<5}", number = 1); //10000


}   