const PI:f32 = 3.1415;
static mut GLOBAL:u8 = 1;

fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a+b);
    a + b
}


fn escopo() {
    println!("PI = {}", PI);

    unsafe {
        println!("variaval_global = {}", GLOBAL);
    }
    
    let inteiro:i32 = 300;
    println!("inteiro = {}, tamanho = {} bytes", inteiro, std::mem::size_of_val(&inteiro));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let mut booleana:bool = true;
    println!("bool = {}, tamanho = {} bytes", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'R';
    println!("letra = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}


fn condicionais() {
    let idade: u8 = 17;
    let responsavel_autorizou = true;

    if idade > 18 {
        println!("pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("pode entrar com assinatura do responsavel");
    } else {
        println!("não pode entrar na balada");
    }

    //modo verboso
    let condicao;
     if idade > 18 {
         condicao = "maior";
     } else {
         condicao = "menor";
     }

     println!("é {} de idade", condicao);


     // modo simples
     let condicao1 = if idade > 18 {"maior"} else {"menor"};

     println!("é {} de idade", condicao1);
}

fn repeticoes(){
    let multiplicador:u8 = 5;
    let mut contador:u8 = 0;
    
    while contador < 10 {
        contador += 1;

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;

    loop {
        contador += 1;

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break;
        }
    }
    
    for i in 1..=10 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }

}

fn main() {
    soma(2, 3);
    escopo();
    condicionais();
    repeticoes();

}
