fn main(){
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(5);

    println!("{}",x);
}

fn takes_ownership(some_str:String){
    println!("{}",some_str);
}

fn makes_copy(some_int:i32){
    println!("{}",some_int);
}