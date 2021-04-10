use std::io;
use std::cmp::Ordering;
use rand::Rng;
//ライブラリをスコープに導入する

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}",secret_number);

    loop{
        println!("Please input your guess.");

        //let foo = 5だと変数は不変になる。mutをつけて初めて可変の変数になる。
        let mut guess = String::new();

        //参照もデフォルトでは不変なので、mutをつけて可変にしている
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //関数はResult型を返し、列挙型のOkとErrの値を持つ。これによってエラーハンドリングができる。
            //expectなしでも関数は呼び出せるが、コンパイラに怒られるのできちんと使おう。

        //シャドーイングがされるので、変数の型を変えたいだけのときに別の変数を作る必要がない
        //trimによって両端の空白を除去する。今回は改行を削除。
        //Goみたいに変数のあとに指定することで変数の型を定められる。
        //Parseは文字列を解析して他の型に変換するので、エラーが出やすい。expectを忘れないように。
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your guessed: {}",guess);

        //型推論によってPythonみたいにやってくれる。比較は当然同じ型じゃないとできないようになっている。
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } 
        }
    }
}
