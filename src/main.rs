use oop_rust::caller_trait_object::TryConnect;
use oop_rust::trait_object::{Initialization, MySql, Postgres};

fn main() {
    let mysql = MySql::init(String::from("Dev"), String::from("127.0.0.1:3000"));
    let postgres = Postgres::init(String::from("Dev"), String::from("127.0.0.1:3000"));
    let try_connect = TryConnect::new();
    try_connect.connecting(Box::new(mysql));
    try_connect.connecting(Box::new(postgres));
    


}



