//in C #include<stdio.h>
use std::io;

fn main (){
    //let name: &str = "fabio"; /*inmutavel*/
   // let mut name2: &str = "fabio"; /*mutavel*/


    //integer
    //let age:u64 = 9;
    // u64  unsigned não suporta inteiros negativos
    //let age:i64 = 9;
    // i64  signed  suporta inteiros do tipos negativos
    //i16 pode estar entre -32768 e 32767


    //let peso = 6.7;
    //f64:flutuante

    //let status:bool = false;

   /*
    fn verify_numbers(number1:i32,number2:i32){
        if number1 > number2{
            println!("{} é maior que  que {}",number1,number2);
        }else{
            println!("{} nao é maior que que {}",number1,number2);
        }
    }

    verify_numbers(25,37)
    */

    // input de dados

    fn convert_to_int(data: &String) -> i32{
        let input_data = data.trim().parse::<i32>().unwrap();
        return  input_data
    }

    let mut number1 = String::new();
    let mut get_number_1 =  io::stdin().read_line(&mut number1).expect("error ao ler os dados").to_string();
    let mut number2 = String::new();
    let mut get_number_2 = io::stdin().read_line(&mut number2).expect("error ao ler os dados").to_string();

    let convert_number1 = convert_to_int(&get_number_1);
    let convert_number2 = convert_to_int(&get_number_2);
    if convert_number1 <= convert_number2 {
        println!("é menor");
    }else{
        println!("não é menor");
    }




}



