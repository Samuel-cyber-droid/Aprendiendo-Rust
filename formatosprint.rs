fr main(){
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
    println!("Base 10:          {}", 69420);

}