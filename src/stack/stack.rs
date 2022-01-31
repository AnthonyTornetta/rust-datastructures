pub mod stack
{
    use std::mem;

    pub struct Stack
    {
        head: Option<Box<Node>>
    }

    struct Node
    {
        value: i32,
        next: Option<Box<Node>>
    }

    impl Stack
    {
        pub fn new() -> Stack
        {
            Stack { head: None }
        }

        pub fn push(&mut self, value: i32)
        {
            let prev = mem::replace(&mut self.head, None);

            self.head = Some(Box::new(Node { value: value, next: prev }));
        }

        pub fn pop(&mut self) -> Option<i32>
        {
            match self.head
            {
                None =>
                {
                    None
                }
                Some(..) =>
                {
                    let prev = mem::replace(&mut self.head, None).unwrap();
                    self.head = prev.next;
                    Some(prev.value)
                }
            }
        }
    }
}