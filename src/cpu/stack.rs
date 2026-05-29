//! Stack module for CHIP-8 emulator.
//!
//! CHIP-8 has a 16-level stack for subroutine calls.
//! Each entry is a 16-bit address (u16).

/// Maximum stack depth (16 levels)
pub const STACK_SIZE: usize = 16;

/// The CHIP-8 stack structure.
#[derive(Debug)]
pub struct Stack {
    /// Stack data - array of 16 addresses
    data: [u16; STACK_SIZE],
    /// Stack pointer - points to the next free slot (0-16)
    sp: usize,
}

impl Stack {
    /// Create a new, empty stack.
    pub fn new() -> Self {
        Stack {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }

    /// Push an address onto the stack.
    ///
    /// # Panics
    /// Panics if the stack overflows (more than 16 entries).
    pub fn push(&mut self, value: u16) {
        assert!(self.sp < STACK_SIZE, "Stack overflow!");
        self.data[self.sp] = value;
        self.sp += 1;
    }

    /// Pop an address from the stack.
    ///
    /// # Panics
    /// Panics if the stack is empty.
    pub fn pop(&mut self) -> u16 {
        assert!(self.sp > 0, "Stack underflow!");
        self.sp -= 1;
        self.data[self.sp]
    }

    /// Reset the stack (clear all entries and reset SP to 0).
    pub fn reset(&mut self) {
        self.data = [0; STACK_SIZE];
        self.sp = 0;
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.sp == 0
    }

    /// Check if the stack is full.
    pub fn is_full(&self) -> bool {
        self.sp == STACK_SIZE
    }

    /// Get the current stack pointer value.
    pub fn sp(&self) -> usize {
        self.sp
    }

    /// Get a reference to the stack data (for debugging).
    pub fn data(&self) -> &[u16; STACK_SIZE] {
        &self.data
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();

        stack.push(0x200);
        assert_eq!(stack.sp(), 1);
        assert_eq!(stack.pop(), 0x200);
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_push_pop_multiple() {
        let mut stack = Stack::new();

        stack.push(0x200);
        stack.push(0x300);
        stack.push(0x400);

        assert_eq!(stack.sp(), 3);
        assert_eq!(stack.pop(), 0x400); // LIFO - last in, first out
        assert_eq!(stack.pop(), 0x300);
        assert_eq!(stack.pop(), 0x200);
    }

    #[test]
    fn test_stack_is_lifo() {
        let mut stack = Stack::new();

        // Push addresses in order
        stack.push(0x100);
        stack.push(0x200);
        stack.push(0x300);
        stack.push(0x400);

        // Pop in reverse order
        assert_eq!(stack.pop(), 0x400); // Last pushed = first popped
        assert_eq!(stack.pop(), 0x300);
        assert_eq!(stack.pop(), 0x200);
        assert_eq!(stack.pop(), 0x100);
    }

    #[test]
    fn test_reset() {
        let mut stack = Stack::new();

        stack.push(0x200);
        stack.push(0x300);
        assert_eq!(stack.sp(), 2);

        stack.reset();
        assert!(stack.is_empty());
        assert_eq!(stack.sp(), 0);
    }

    #[test]
    fn test_is_full() {
        let mut stack = Stack::new();

        assert!(!stack.is_full());

        // Push 16 entries to fill the stack
        for i in 0..STACK_SIZE {
            stack.push(i as u16);
        }

        assert!(stack.is_full());
        assert_eq!(stack.sp(), STACK_SIZE);
    }

    #[test]
    #[should_panic]
    fn test_overflow() {
        let mut stack = Stack::new();

        // Push 17 entries (overflow on the 17th)
        for i in 0..(STACK_SIZE + 1) {
            stack.push(i as u16);
        }
    }

    #[test]
    #[should_panic]
    fn test_underflow() {
        let mut stack = Stack::new();
        stack.pop(); // Pop from empty stack
    }
}
