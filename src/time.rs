//! Time units

use crate::rcc::Clocks;
use cortex_m::peripheral::DWT;

/// Bits per second
#[derive(Clone, Copy, Debug)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy, Debug)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy, Debug)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy, Debug)]
pub struct MegaHertz(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `MilliSeconds`
    fn milliseconds(self) -> MilliSeconds;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn milliseconds(self) -> MilliSeconds {
        MilliSeconds(self)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

impl From<MilliSeconds> for Hertz {
    fn from(ms: MilliSeconds) -> Self {
        if ms.0 <= 1000 {
            Hertz((1000 + ms.0 / 2) / ms.0)
        } else {
            Hertz(1)
        }
    }
}

/// A monotonic nondecreasing timer
#[derive(Clone, Copy, Debug)]
pub struct MonoTimer {
    frequency: Hertz,
}

impl MonoTimer {
    /// Creates a new `Monotonic` timer
    pub fn new(mut dwt: DWT, clocks: Clocks) -> Self {
        dwt.enable_cycle_counter();

        // now the CYCCNT counter can't be stopped or resetted
        drop(dwt);

        MonoTimer {
            frequency: clocks.sysclk(),
        }
    }

    /// Returns the frequency at which the monotonic timer is operating at
    pub fn frequency(&self) -> Hertz {
        self.frequency
    }

    /// Returns an `Instant` corresponding to "now"
    pub fn now(&self) -> Instant {
        Instant {
            now: DWT::get_cycle_count(),
        }
    }
}

/// A measurement of a monotonically nondecreasing clock
#[derive(Clone, Copy, Debug)]
pub struct Instant {
    now: u32,
}

impl Instant {
    /// Ticks elapsed since the `Instant` was created
    pub fn elapsed(&self) -> u32 {
        DWT::get_cycle_count().wrapping_sub(self.now)
    }
}

/// Time unit
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct MilliSeconds(pub u32);
