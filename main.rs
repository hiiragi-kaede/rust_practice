fn main(){
    let user = build_user(String::from("aaa.com"), String::from("tas"));

    println!("{}:{}",user.email,user.username);
}

struct User{
    email: String,
    username: String
}

fn build_user(Email:String, Username: String)-> User{
    User{
        Email,
        Username
    }
}