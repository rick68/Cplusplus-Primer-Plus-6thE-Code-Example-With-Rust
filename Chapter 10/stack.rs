// stack.rs -- Stack member functions

pub type Item = u64;

const MAX: usize = 10;

pub struct Stack
{
    top: usize,
    items: [Item; MAX],
}

impl Stack
{
    pub fn new() -> Stack {
        Stack {
            items: [Item::default(); MAX],
            top: 0,
        }
    }

    pub fn isempty(&self) -> bool {
        self.top == 0
    }

    pub fn isfull(&self) -> bool {
        self.top == MAX
    }

    pub fn push(&mut self, item: &Item) -> bool {
        if self.top < MAX {
            self.items[self.top] = *item;
            self.top += 1;
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self, item: &mut Item) -> bool {
        if self.top > 0 {
            self.top -= 1;
            *item = self.items[self.top];
            true
        } else {
            false
        }
    }
}
