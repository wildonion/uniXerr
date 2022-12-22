




// NOTE - use pytorch geometric and dgl.ai


mod GNN{
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    //-- bypass cyclic references using Weak<T>
    struct Neuron{
        //-- neuron is a feature
        //-- we want to have a reference to the data vector and also change its values inside of it
        //-- variables are immutable by default so we're using RefCell in order to change the values
        //-- of data vector at runtime and Rc in order to have multiple references to it from other Neurons.
        data: RefCell<Rc<Vec<f64>>>,
        parent: RefCell<Weak<Neuron>>,
        children: RefCell<Vec<Rc<Neuron>>>,
    }
}
