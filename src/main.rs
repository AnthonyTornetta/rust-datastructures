mod stack;

use crate::stack::stack::stack::Stack;

fn main()
{
    let mut stack = Stack::new();
    
    stack.push(5);
    stack.push(4);
    stack.push(3);
    stack.push(2);
    stack.push(1);
    stack.push(0);

    let mut elem = stack.pop();
    while !elem.is_none()
    {
        println!("{}", elem.unwrap());
        elem = stack.pop();
    }
}
