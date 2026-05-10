use super::{Core, Inside, InsideMut, has};

impl<'a, C: Core + has::Audio> Inside<'a, C> {
    /// Borrow the core's audio.
    #[must_use]
    pub fn audio(self) -> &'a C::Audio {
        self.0.audio()
    }
}

impl<'a, C: Core + has::Audio> InsideMut<'a, C> {
    /// Mutably borrow the core's audio.
    #[must_use]
    pub fn audio(self) -> &'a mut C::Audio {
        self.0.audio_mut()
    }
}

impl<'a, C: Core + has::Input> Inside<'a, C> {
    /// Borrow the core's input.
    #[must_use]
    pub fn input(self) -> &'a C::Input {
        self.0.input()
    }
}

impl<'a, C: Core + has::Input> InsideMut<'a, C> {
    /// Mutably borrow the core's input.
    #[must_use]
    pub fn input(self) -> &'a mut C::Input {
        self.0.input_mut()
    }
}

impl<'a, C: Core + has::Processor> Inside<'a, C> {
    /// Borrow the core's processor.
    #[must_use]
    pub fn proc(self) -> &'a C::Proc {
        self.0.proc()
    }
}

impl<'a, C: Core + has::Processor> InsideMut<'a, C> {
    /// Mutably borrow the core's processor.
    #[must_use]
    pub fn proc(self) -> &'a mut C::Proc {
        self.0.proc_mut()
    }
}

impl<'a, C: Core + has::Cable> Inside<'a, C> {
    /// Borrow the core's cable.
    #[must_use]
    pub fn cable(self) -> &'a C::Cable {
        self.0.cable()
    }
}

impl<'a, C: Core + has::Cable> InsideMut<'a, C> {
    /// Mutably borrow the core's cable.
    #[must_use]
    pub fn cable(self) -> &'a mut C::Cable {
        self.0.cable_mut()
    }
}

impl<'a, C: Core + has::Video> Inside<'a, C> {
    /// Borrow the core's video.
    #[must_use]
    pub fn video(self) -> &'a C::Video {
        self.0.video()
    }
}

impl<'a, C: Core + has::Video> InsideMut<'a, C> {
    /// Mutably borrow the core's video.
    #[must_use]
    pub fn video(self) -> &'a mut C::Video {
        self.0.video_mut()
    }
}
