pub mod trait_object {
    
    pub trait Connection {
        fn connect(&self);
    }

    pub trait Initialization {
        
        fn init(name : String, port : String) -> Self;
        fn check(&self);

    }

    #[derive(Debug)]
    pub struct Postgres {
         name : String,
         port : String
    }


    #[derive(Debug)]
    pub struct MySql {
        name : String,
        port : String
    }

    impl Connection for Postgres {
        fn connect(&self) {
            println!("Connection Postgres");
        }
    }

    impl Initialization for Postgres {

        fn init(name : String, port : String) -> Self {
            Self { name, port }
        }

        fn check(&self) {
            println!("name {}", self.name);
            println!("port {}", self.port);
        }

    }


    impl Initialization for MySql {

        fn init(name : String, port : String) -> Self {
            Self { name, port }
        }

        fn check(&self) {
            println!("name {}", self.name);
            println!("port {}", self.port);
        }
        
    }

    impl Connection for MySql {
        fn connect(&self) {
            println!("Connecting MySql");
        }
    }
    
}

pub mod caller_trait_object {
    use crate::trait_object::Connection;

    pub struct TryConnect;
    
    impl TryConnect {

        pub fn new() -> Self {
            Self
        }

        pub fn connecting(&self,trt_obj : Box<dyn Connection>) {
            trt_obj.connect();
        }
    }

}