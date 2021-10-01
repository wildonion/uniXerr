






pub fn lib_func_sample(){}


pub trait Info{
    fn who(&self) -> String; //-- this is not object safe trait cause who is an associated method means it has self parameter
}


pub mod Preprocessing{}
