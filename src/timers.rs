//! Timers module for CHIP-8 emulator.
//!
//! CHIP-8 has two timers: delay and sound.
//! Both are 8-bit values that count down at 60Hz.

/// Timer frequency in Hz (cycles per second)
pub const TIMER_FREQUENCY: u32 = 60;

/// The CHIP-8 timers structure.
#[derive(Debug, Default)]
pub struct Timers {
    /// Delay timer - used to time animations/events
    pub delay: u8,
    /// Sound timer - triggers beep when > 0
    pub sound: u8,
}

impl Timers {
    /// Create new timers with both set to 0.
    pub fn new() -> Self {
        Timers {
            delay: 0,
            sound: 0,
        }
    }

    /// Reset both timers to 0.
    pub fn reset(&mut self) {
        self.delay = 0;
        self.sound = 0;
    }

    /// Set the delay timer to a value.
    pub fn set_delay(&mut self, value: u8) {
        self.delay = value;
    }

    /// Set the sound timer to a value.
    pub fn set_sound(&mut self, value: u8) {
        self.sound = value;
    }

    /// Get the current delay timer value.
    pub fn get_delay(&self) -> u8 {
        self.delay
    }

    /// Get the current sound timer value.
    pub fn get_sound(&self) -> u8 {
        self.sound
    }

    /// Check if sound should be playing.
    pub fn is_sound_active(&self) -> bool {
        self.sound > 0
    }

    /// Tick the timers (call 60 times per second).
    ///
    /// Decrements both timers if they are greater than 0.
    /// Returns true if sound should play (sound timer > 0).
    pub fn tick(&mut self) -> bool {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
        self.is_sound_active()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_timers_are_zero() {
        let timers = Timers::new();
        assert_eq!(timers.delay, 0);
        assert_eq!(timers.sound, 0);
    }

    #[test]
    fn test_set_delay() {
        let mut timers = Timers::new();
        timers.set_delay(100);
        assert_eq!(timers.get_delay(), 100);
    }

    #[test]
    fn test_set_sound() {
        let mut timers = Timers::new();
        timers.set_sound(50);
        assert_eq!(timers.get_sound(), 50);
    }

    #[test]
    fn test_reset() {
        let mut timers = Timers::new();
        timers.set_delay(100);
        timers.set_sound(50);
        timers.reset();
        assert_eq!(timers.delay, 0);
        assert_eq!(timers.sound, 0);
    }

    #[test]
    fn test_tick_decrements_both() {
        let mut timers = Timers::new();
        timers.set_delay(10);
        timers.set_sound(5);
        
        timers.tick();
        assert_eq!(timers.delay, 9);
        assert_eq!(timers.sound, 4);
    }

    #[test]
    fn test_tick_stops_at_zero() {
        let mut timers = Timers::new();
        timers.set_delay(1);
        timers.set_sound(1);
        
        timers.tick();
        assert_eq!(timers.delay, 0);
        assert_eq!(timers.sound, 0);
        
        // Should not go negative
        timers.tick();
        assert_eq!(timers.delay, 0);
        assert_eq!(timers.sound, 0);
    }

    #[test]
    fn test_is_sound_active() {
        let mut timers = Timers::new();
        assert!(!timers.is_sound_active());
        
        timers.set_sound(1);
        assert!(timers.is_sound_active());
        
        timers.tick();
        assert!(!timers.is_sound_active());
    }
}
