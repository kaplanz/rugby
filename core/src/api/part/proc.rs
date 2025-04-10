//! Processor API.

/// Processor interface.
pub trait Processor {
    /// Instruction Set Architecture (ISA).
    ///
    /// Represents a valid instance of an instruction in the processor's ISA.
    type Insn;

    /// Gets the current instruction.
    fn insn(&self) -> Self::Insn;

    /// Move the PC to the provided address.
    fn goto(&mut self, pc: u16);

    /// Execute the provided instruction in-place.
    fn exec(&mut self, code: u8);

    /// Run the provided program (i.e. instruction sequence) in-place.
    fn run(&mut self, prog: &[u8]);

    /// Enable (or wake) the processor.
    fn wake(&mut self);
}
