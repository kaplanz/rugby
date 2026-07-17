//! Hardware timer.

use log::{debug, trace};
use rugby_arch::mio::{Bus, Mmio};
use rugby_arch::reg::{Port, Register};
use rugby_arch::{Block, Shared};

use super::irq::{self, Interrupt};

/// Timer register select.
///
/// See more details [here][regs].
///
/// [regs]: https://gbdev.io/pandocs/Timer_and_Divider_Registers.html
#[derive(Copy, Clone, Debug)]
pub enum Select {
    /// `[$FF04]`: Divider register.
    Div,
    /// `[$FF05]`: Timer counter.
    Tima,
    /// `[$FF06]`: Timer modulo.
    Tma,
    /// `[$FF07]`: Timer control.
    Tac,
}

/// Hardware timer.
#[derive(Debug)]
pub struct Timer {
    /// Timer registers.
    pub reg: File,
    /// Timer internals.
    pub etc: Internal,
    /// Interrupt line.
    pub int: irq::Line,
}

/// Timer internals.
#[derive(Debug, Default)]
pub struct Internal {
    /// Previous AND result.
    and: bool,
}

impl Internal {
    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Timer {
    /// Constructs a new `Timer`.
    #[must_use]
    pub fn new(int: irq::Line) -> Self {
        Self {
            reg: File::default(),
            etc: Internal::default(),
            int,
        }
    }

    /// Calculates the AND result.
    ///
    /// Used to determine whether TIMA will be incremented, as documented by
    /// [Hacktix][gbedg].
    ///
    /// [gbedg]: https://github.com/Hacktix/GBEDG/blob/master/timers/index.md#timer-operation
    fn andres(&self) -> bool {
        let ena = self.reg.tac.borrow().ena();
        let sel = self.reg.tac.borrow().select();
        let div = self.reg.div.borrow().div();
        ena && (sel & div != 0)
    }
}

impl Block for Timer {
    #[rustfmt::skip]
    fn cycle(&mut self) {
        // Increment the divider every T-cycle.
        //
        // Since only the upper 8-bits of DIV are mapped, has the observable
        // effect of incrementing DIV (as read by the CPU) every 256 T-cycles.
        self.reg.div.borrow_mut().inc();

        // Reload TIMA
        let rel = self.reg.tima.borrow().rel;
        // Request an interrupt
        if rel == reg::Reload::Load {
            debug!("timer reloaded");
            self.int.raise(Interrupt::Timer);
        }
        // Commit the reload
        if matches!(rel, reg::Reload::Load | reg::Reload::Done(1..)) {
            if self.reg.tima.borrow().sup {
                // A write to TIMA during the delay suppresses the copy. The
                // written value is kept and the commit window is skipped.
                self.reg.tima.borrow_mut().rel = reg::Reload::None;
                self.reg.tima.borrow_mut().sup = false;
            } else {
                // Copy TMA into TIMA. Copying continuously for the entire
                // window means writes to TMA during the reload cycle also
                // propagate into TIMA.
                let tma = self.reg.tma.load();
                self.reg.tima.borrow_mut().reg = tma;
            }
        }
        self.reg.tima.borrow_mut().rel.tick();

        // Check if TIMA should be incremented
        let this = self.andres();         // calculate AND result
        let tick = self.etc.and && !this; // check for falling edge
        self.etc.and = this;              // store for next cycle

        // Increment TIMA
        if tick {
            let carry = self.reg.tima.borrow_mut().inc();
            trace!("timer: {}", self.reg.tima.load());
            // Trigger pending reload on overflow
            if carry {
                self.reg.tima.borrow_mut().rel.sched();
                debug!("timer reload pending");
            }
        }
    }

    fn reset(&mut self) {
        self.reg.reset();
        self.etc.reset();
    }
}

impl Mmio for Timer {
    fn attach(&self, bus: &mut Bus) {
        self.reg.attach(bus);
    }
}

impl Port<u8> for Timer {
    type Select = Select;

    fn load(&self, reg: Self::Select) -> u8 {
        match reg {
            Select::Div => self.reg.div.load(),
            Select::Tima => self.reg.tima.load(),
            Select::Tma => self.reg.tma.load(),
            Select::Tac => self.reg.tac.load(),
        }
    }

    fn store(&mut self, reg: Self::Select, value: u8) {
        match reg {
            Select::Div => self.reg.div.store(value),
            Select::Tima => self.reg.tima.store(value),
            Select::Tma => self.reg.tma.store(value),
            Select::Tac => self.reg.tac.store(value),
        }
    }
}

/// Timer registers.
///
/// | Address | Size | Name | Description      |
/// |:-------:|------|------|------------------|
/// | `$FF04` | Byte | DIV  | Divider register |
/// | `$FF05` | Byte | TIMA | Timer counter    |
/// | `$FF06` | Byte | TMA  | Timer modulo     |
/// | `$FF07` | Byte | TAC  | Timer control    |
#[derive(Debug, Default)]
pub struct File {
    /// Divider register.
    pub div: Shared<reg::Div>,
    /// Timer counter.
    pub tima: Shared<reg::Tima>,
    /// Timer modulo.
    pub tma: Shared<reg::Tma>,
    /// Timer control.
    pub tac: Shared<reg::Tac>,
}

impl Block for File {
    fn reset(&mut self) {
        self.div.take();
        self.tima.take();
        self.tma.take();
        self.tac.take();
    }
}

impl Mmio for File {
    fn attach(&self, bus: &mut Bus) {
        bus.map(0xff04..=0xff04, self.div.clone().into());
        bus.map(0xff05..=0xff05, self.tima.clone().into());
        bus.map(0xff06..=0xff06, self.tma.clone().into());
        bus.map(0xff07..=0xff07, self.tac.clone().into());
    }
}

/// Timer register models.
pub mod reg;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tima_reload_works() {
        // Configure 65536 Hz timer (64 cycles)
        let mut timer = Timer::new(irq::Irq::default().line);
        timer.reg.tac.store(0b110);
        timer.reg.tma.store(0xfe);
        timer.reg.tima.store(0xfe);
        assert_eq!(timer.reg.tima.load(), 0xfe);

        // hold   -> 0xfe
        for _ in 1..64 {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xfe);
        }
        // cycle  -> 0xff
        {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xff);
        }
        // hold   -> 0xff
        for _ in 1..64 {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xff);
        }
        // wrap   -> 0x00
        {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0x00);
        }
        // delay  -> 0x00
        for _ in 1..4 {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0x00);
        }
        // reload -> 0xfe
        {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xfe);
        }
        // hold   -> 0xfe
        for _ in 5..64 {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xfe);
        }
        // cycle  -> 0xff
        {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xff);
        }
        // hold   -> 0xff
        for _ in 1..64 {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xff);
        }
        // wrap   -> 0x00
        {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0x00);
        }
        // delay  -> 0x00
        for _ in 1..4 {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0x00);
        }
        // reload -> 0xfe
        {
            timer.cycle();
            assert_eq!(timer.reg.tima.load(), 0xfe);
        }
    }

    #[test]
    fn tima_write_reloading_working() {
        /// Runs a write-reloading test, storing `$FD` to `TIMA` after
        /// `offset` delay cycles once the overflow has occurred.
        fn run(line: &irq::Line, offset: usize) {
            // Configure 65536 Hz timer (64 cycles)
            let mut timer = Timer::new(line.clone());
            timer.reg.tac.store(0b110);
            timer.reg.tma.store(0xfe);
            timer.reg.tima.store(0xfe);
            assert_eq!(timer.reg.tima.load(), 0xfe);

            // hold   -> 0xfe
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // cycle  -> 0xff
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // hold   -> 0xff
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // wrap   -> 0x00
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // delay  -> 0x00
            for _ in 0..offset {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // write  -> 0xfd (reload cancelled)
            {
                timer.reg.tima.store(0xfd);
                assert_eq!(timer.reg.tima.load(), 0xfd);
            }
            // hold   -> 0xfd
            for _ in offset..63 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfd);
            }
            // cycle  -> 0xfe
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
        }

        let line = irq::Irq::default().line;
        // Write during each cycle of the reload delay
        run(&line, 0);
        run(&line, 1);
        run(&line, 2);
        run(&line, 3);

        // Test 5: write after the reload has occurred
        {
            // Configure 65536 Hz timer (64 cycles)
            let mut timer = Timer::new(line.clone());
            timer.reg.tac.store(0b110);
            timer.reg.tma.store(0xfe);
            timer.reg.tima.store(0xfe);
            assert_eq!(timer.reg.tima.load(), 0xfe);

            // hold   -> 0xfe
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // cycle  -> 0xff
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // hold   -> 0xff
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // wrap   -> 0x00
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // delay  -> 0x00
            for _ in 1..4 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // reload -> 0xfe
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // write  -> 0xfe (ignored!)
            {
                timer.reg.tima.store(0xfd);
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // hold   -> 0xfe
            for _ in 5..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // cycle  -> 0xff
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
        }
    }

    #[test]
    fn tma_write_reloading_working() {
        /// Runs a write-reloading test, storing `$69` to `TMA` after
        /// `offset` delay cycles once the overflow has occurred.
        fn run(line: &irq::Line, offset: usize) {
            // Configure 65536 Hz timer (64 cycles)
            let mut timer = Timer::new(line.clone());
            timer.reg.tac.store(0b110);
            timer.reg.tma.store(0xfe);
            timer.reg.tima.store(0xfe);
            assert_eq!(timer.reg.tima.load(), 0xfe);

            // hold   -> 0xfe
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // cycle  -> 0xff
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // hold   -> 0xff
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // wrap   -> 0x00
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // delay  -> 0x00
            for _ in 0..offset {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // write  -> 0x69 (into TMA)
            {
                timer.reg.tma.store(0x69);
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // delay  -> 0x00
            for _ in offset..3 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // reload -> 0x69
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x69);
            }
            // hold   -> 0x69
            for _ in 4..63 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x69);
            }
            // cycle  -> 0x6a
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x6a);
            }
        }

        let line = irq::Irq::default().line;
        // Write during each cycle of the reload delay
        run(&line, 0);
        run(&line, 1);
        run(&line, 2);
        run(&line, 3);

        // Test 5: write while the reload is still committing
        {
            // Configure 65536 Hz timer (64 cycles)
            let mut timer = Timer::new(line.clone());
            timer.reg.tac.store(0b110);
            timer.reg.tma.store(0xfe);
            timer.reg.tima.store(0xfe);
            assert_eq!(timer.reg.tima.load(), 0xfe);

            // hold   -> 0xfe
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // cycle  -> 0xff
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // hold   -> 0xff
            for _ in 1..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xff);
            }
            // wrap   -> 0x00
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // delay  -> 0x00
            for _ in 1..4 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x00);
            }
            // reload -> 0xfe
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // write  -> 0x69 (still committing!)
            {
                timer.reg.tma.store(0x69);
                assert_eq!(timer.reg.tima.load(), 0xfe);
            }
            // copy   -> 0x69
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x69);
            }
            // hold   -> 0x69
            for _ in 6..64 {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x69);
            }
            // cycle  -> 0x6a
            {
                timer.cycle();
                assert_eq!(timer.reg.tima.load(), 0x6a);
            }
        }
    }
}
