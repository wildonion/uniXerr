






pub fn lib_func_sample(){}


pub trait Info{
    fn what(&self) -> String; //-- this is not object safe trait cause who is an associated method means it has self parameter
}


pub mod Preprocessing{
    
    pub mod Scaler{
        
        pub async fn minmax(){}
        pub async fn standard(){}
    
    }
    
}


pub mod DataLoader{
    
    pub async fn mean(){}
    pub async fn std(){}
    
}
