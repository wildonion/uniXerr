



use std::cell::RefCell;
use std::rc::{Rc, Weak};



fn test_graph(){
            
    
            //-- Rc<T> , RefCell<T> , Weak<T> example
            //-- using a graph and tree structure
            //---------------------------------------
            #[derive(Debug)]
            struct Node{
                value: i32,
                parent: RefCell<Weak<Node>>,
                children: RefCell<Vec<Rc<Node>>>,
            }



            let leaf = Rc::new(Node{
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });


            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf), //-- 1 , a reference to leaf Rc<Node>
                Rc::weak_count(&leaf), //-- 0 , because nothing is pointing to leaf thus no owner or reference is available for leaf.
            );

            
                // -------------------- NEW SCOPE ------------------------
                    {
                        let branch = Rc::new(Node {
                            value: 5,
                            parent: RefCell::new(Weak::new()),
                            children: RefCell::new(vec![Rc::clone(&leaf)]), //-- a tree could have strong Rc pointers from parent nodes to children
                        });

                        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //-- leaf is pointing to the branch parent thus the branch node has an owner or reference, Weak pointers from children back to their parents

                        println!(
                            "branch strong = {}, weak = {}",
                            Rc::strong_count(&branch), //-- 1 , a reference to baranch Rc<Node>
                            Rc::weak_count(&branch), //-- 1 , because leaf is pointing to the branch
                        );

                        println!(
                            "leaf strong = {}, weak = {}",
                            Rc::strong_count(&leaf), //-- 2 , because leaf is the branch children thus a reference is pointing to the leaf and it'll increament the strong_count
                            Rc::weak_count(&leaf), //-- 0
                        );
                    }
                // -------------------- END OF NEW SCOPE ------------------------

            
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf), //-- 1 , because we're out of the inner scope and we don't have the branch parent in here to reference to the leaf node child
                Rc::weak_count(&leaf), //-- 0
            );


            
            ////// ========================== GNN CODES ==========================


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

}
