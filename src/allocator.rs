pub struct MemFreeBlock {
    pub next: Option<Box<MemFreeBlock>>, // Pointer to the next free block (linked list)
    pub size: usize,                     // Size of the free block
}

impl MemFreeBlock {
    pub fn new() -> Self {
        MemFreeBlock {
            size: 0,
            next: None,
        }
    }
    // Get a reference to the next block
    pub fn get_next(&self) -> Option<&Box<MemFreeBlock>> {
        self.next.as_ref() // Return a reference to the next block
    }

    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }

    // Set the next block
    pub fn set_next(&mut self, n: Option<Box<MemFreeBlock>>) {
        self.next = n;
    }

    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }
}

struct MemMetaBlock {
    pub size: usize, // Size of the block
}

impl MemMetaBlock {
    pub fn new(size: usize) -> Self {
        MemMetaBlock { size }
    }
    // Get the size of the block
    pub fn get_size(&self) -> usize {
        self.size
    }
    // Set the size of the block
    pub fn set_size(&mut self, s: usize) {
        self.size = s;
    }
}
