use std::rc::Rc;
use crate::tree::*;
use std::cell::RefCell;

// translated from RevBayes
// core/io/NewickConverter.cpp

#[test]
pub fn test_the_thing(){
    let s = "((A:0.5,B:0.5):0.9,C:0.8);".to_string();

    let root = Rc::new(Node2 {
        index: 1,
        label: RefCell::new("".to_string()),
        //label: species_name.to_string(),
        children: RefCell::new(vec![]),
    });

    create_node(&s, &root);
}

pub fn create_node(s: &str, parent_node: &Rc<Node2>){
    const OPENING_TOKENS: &[char] = &[':', '[', ';', ',', ')'];

    let mut stream = s
        .chars()
        .peekable();

    let mut c = stream
        .next()
        .expect("should have been able to read char");

    if c != '('{
        panic!("expected opening parenthesis");
    }
   
    let node = Rc::new(Node2 {
        index: 1,
        label: RefCell::new("".to_string()),
        //label: species_name.to_string(),
        children: RefCell::new(vec![]),
    });

 
    while stream.peek().expect("asd") != &')'{

        if stream.peek().expect("could not read next") == &'('{
            // internal node
            let mut depth = 0;
            let mut child_string= "".to_string(); 

            loop{
                c = stream.next().expect("2");
                child_string.push(c);
                if c == '('{
                    depth += 1;
                }
                else if c == ')'{
                    depth -= 1;
                }

                if depth == 0 {
                    break
                }
            }
            // construct child node
            create_node(&child_string, &node);
        }else{
            
        }

        // add node as a child of the parent node
        parent_node.children.borrow_mut().push(Rc::clone(&node));

        // read label
        let mut label = "".to_string();
        while !OPENING_TOKENS.contains(&stream.peek().expect("could not peek1")) {
           label.push(stream.next().expect("could not read next char")); 
        }
        *node.label.borrow_mut() = label;

        // read branch length
        if stream.peek().expect("could not peek2") == &':'{
            loop{
                let _ = stream.next();

                let p = stream.peek().expect("could not peek3");
                if (p != &';') & (p != &',') & (p != &')') & (p != &'['){
                    break
                }
            }
        }

        /*
        // ignore the comments
        if stream.peek().expect("could not peek") == &'['{
            loop{
                let _ = stream.next();

                if stream.peek().expect("could not peek") == &']'{
                    break
                }
            }
        }
        */

        // skip comma
        if *stream.peek().expect("could not peek4") == ','{
            stream.next();
        }

        // avoid infinite loop
        if *stream.peek().expect("could not peek5") == ';'{
            //let _ = stream.next();
            panic!("not enough closing parentheses");
        }
    }

    // remove closing parenthesis
    let _ = stream.next();

    let n = stream.peek();
    println!("is some in next: {:?}", n);
    if n == None {
        println!("horse");
    }

    // read optional label, check for EOF
    let mut label = "".to_string();  
    loop{
        let p = stream.peek().expect("could not peek6");
        if (p != &':') & (p != &';') & (p != &',') & (p != &'[') & (p != &(0xff as char)){
            break
        }

        c = stream.next().expect("could not get next");
        label.push(c); 
    }
    *node.label.borrow_mut() = label;

    // read branch length
    let p = stream.peek().expect("should have been able to peek");
    if p == &':'{
        stream.next();

        loop{
            //let _ = stream.next();
            //c = stream.next().expect("should have been able to get next");
            stream.next();

            let p = stream.peek().expect("should have been able to peek");

            if (p != &';') & (p != &',') & (p != &'['){
                break
            }
        }
    }

    //let c: char =
    // let c = stream

    //tokens
}
