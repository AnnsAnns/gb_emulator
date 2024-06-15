use super::CPU;

const DMA_REGISTER_ADDR: u16 = 0xFF46;
const OAM_BASE: u16 = 0xFE00;

impl CPU {
    /// Handles the DMA circuit
    /// See: https://gbdev.io/pandocs/OAM_DMA_Transfer.html
    pub fn dma_routine(&mut self) {
        if !self.dma_active {
            return;
        }

        // Get DMA base from memory
        let dma_base = (self.memory.read_byte(DMA_REGISTER_ADDR) as u16) << 8;
        
        // Get the current byte to be read by combining the base + dma_current_offset
        let source_addr = dma_base + self.dma_current_offset as u16;
        let target_addr = OAM_BASE + self.dma_current_offset as u16;
        
        // Write from memory to OAM
        self.memory.write_byte(target_addr, self.memory.read_byte(source_addr));

        // Append offset
        let (_, overflow) = self.dma_current_offset.overflowing_add(1);

        // If the offset overflows we reached the end
        if overflow {
            self.dma_active = false;
            self.dma_current_offset = 0;
        } else {
            self.dma_current_offset += 1;
        }
    }
}