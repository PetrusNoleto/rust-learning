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

    fn verify_numbers(number1:i32,number2:i32){
        if number1 > number2{
            println!("{} é maior que  que {}",number1,number2);
        }else{
            println!("{} nao é maior que que {}",number1,number2);
        }
    }

    verify_numbers(25,37)










}



