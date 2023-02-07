



# define the coiniXerr transaction structure 
# ...


struct Transaction{
    id 0@: Text;
    published: 1@: Date;
}

interface Transaction {
    tx @0 (x :Int32) -> (y :Int32);
}