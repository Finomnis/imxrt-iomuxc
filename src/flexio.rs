//! FlexIO pad configurations

/// A FlexIO pin
pub trait Pin<const Module: u8>: super::Iomuxc {
    // The module needs to be a generic because some pads
    // are attached to multiple FlexIO modules

    /// FlexIO pin number
    type Pin: super::consts::Unsigned;
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: Option<super::Daisy>;
}

/// Prepare a FlexIO pin.
pub fn prepare<const N: u8, P: Pin<N>>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! flexio {
    (module: $module:literal, pin: $pin:ty, pad: $pad:ty, alt: $alt:expr, daisy: $daisy:expr) => {
        impl Pin<$module> for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            type Pin = $pin;
        }
    };
}
