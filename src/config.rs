use super::Duration;

/// [`Button`](super::Button) configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ButtonConfig {
    /// Time the button should be down in order to count it as a press.
    pub debounce: Duration,
    /// Time between consecutive presses to count as a press in the same sequence instead of a new
    /// sequence.
    pub double_click: Duration,
    /// Time the button is held before a long press is detected.
    pub long_press: Duration,
    /// If this is Some(), then a long press will cause short presses to be
    /// emitted repeatedly on this interval until the button is released.
    /// A long press event will also be emitted at the start of the series.
    pub held_repeat_interval: Option<Duration>,
    /// Button direction.
    pub mode: Mode,
}

impl ButtonConfig {
    /// Returns a new [ButtonConfig].
    pub fn new(
        debounce: Duration,
        double_click: Duration,
        long_press: Duration,
        held_repeat_interval: Option<Duration>,
        mode: Mode,
    ) -> Self {
        Self {
            debounce,
            double_click,
            long_press,
            held_repeat_interval,
            mode,
        }
    }
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            debounce: Duration::from_millis(10),
            double_click: Duration::from_millis(350),
            long_press: Duration::from_millis(1000),
            held_repeat_interval: None,
            mode: Mode::default(),
        }
    }
}

/// Button direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    /// Button is connected to a pin with a pull-up resistor. Button pressed it logic 0.
    #[default]
    PullUp,
    /// Button is connected to a pin with a pull-down resistor. Button pressed it logic 1.
    PullDown,
}

impl Mode {
    /// Is button connected to a pin with a pull-up resistor?
    pub const fn is_pullup(&self) -> bool {
        matches!(self, Mode::PullUp)
    }

    /// Is button connected to a pin with a pull-down resistor?
    pub const fn is_pulldown(&self) -> bool {
        !self.is_pullup()
    }
}
