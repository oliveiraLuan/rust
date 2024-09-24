fn main(){
    let mut x = 5;
    println!("O valor de x é {x}");
    x = 6;
    println!("O valor de x agora é {x}");

    shadow();
}

fn shadow(){
    let x = 1;
    
    let x = x + 1;

    println!("X nesse contexto vale {x}");

    {
        let x = x * 2;
        println!("X nesse contexto vale: {x}");
    }
}