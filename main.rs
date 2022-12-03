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

fn main() {
    soma(2, 3);
    escopo();
}
