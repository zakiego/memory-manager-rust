struct MemoryBlock {
    size: usize,
    data: Vec<u8>,
}

impl MemoryBlock {
    fn new(size: usize) -> Self {
        MemoryBlock {
            size,
            data: vec![0; size],
        }
    }
}

struct MemoryManager {
    blocks: Vec<MemoryBlock>,
}

impl MemoryManager {
    fn new() -> Self {
        MemoryManager { blocks: Vec::new() }
    }

    fn allocate(&mut self, size: usize) {
        self.blocks.push(MemoryBlock::new(size));
        println!("Memory allocated successfully!");
    }

    fn deallocate(&mut self, index: usize) {
        if index >= self.blocks.len() {
            println!("Invalid block index!");
            return;
        }

        if self.blocks.is_empty() {
            println!("No memory blocks to deallocate!");
            return;
        }

        self.blocks.remove(index);
        println!("Memory deallocated successfully!")
    }

    fn display_status(&self) {
        println!("Current memory allocation status: ");
        for (i, block) in self.blocks.iter().enumerate() {
            println!("Block {}: Size - {} bytes", i, block.size)
        }
    }
}

fn main() {
    let mut memory_manager = MemoryManager::new();

    // Allocate memory blocks
    memory_manager.allocate(1024);
    memory_manager.allocate(512);
    memory_manager.allocate(2048);

    memory_manager.display_status();

    memory_manager.deallocate(1);

    memory_manager.deallocate(10);

    memory_manager.display_status();
}
