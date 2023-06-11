//! FlexSPI pad configurations

/// A FlexSPI signal
pub trait Signal: private::Sealed {}

/// A tag that indicates a FlexSPI A Data0 pad
pub enum Data0 {}
/// A tag that indicates a FlexSPI A Data1 pad
pub enum Data1 {}
/// A tag that indicates a FlexSPI A Data2 pad
pub enum Data2 {}
/// A tag that indicates a FlexSPI A Data3 pad
pub enum Data3 {}
/// A tag that indicates a FlexSPI A Dqs pad
pub enum Dqs {}
/// A tag that indicates a FlexSPI A Sclk pad
pub enum Sclk {}
/// A tag that indicates a FlexSPI A Ss0b pad
pub enum Ss0b {}
/// A tag that indicates a FlexSPI A Ss1b pad
pub enum Ss1b {}

/// A FlexSPI port; one of `A` or `B`
pub trait Port: private::Sealed {}
/// FlexSPI port A
pub enum A {}
/// FlexSPI port B
pub enum B {}

impl Signal for Data0 {}
impl Signal for Data1 {}
impl Signal for Data2 {}
impl Signal for Data3 {}
impl Signal for Dqs {}
impl Signal for Sclk {}
impl Signal for Ss0b {}
impl Signal for Ss1b {}

impl Port for A {}
impl Port for B {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::Data0 {}
    impl Sealed for super::Data1 {}
    impl Sealed for super::Data2 {}
    impl Sealed for super::Data3 {}
    impl Sealed for super::Dqs {}
    impl Sealed for super::Sclk {}
    impl Sealed for super::Ss0b {}
    impl Sealed for super::Ss1b {}

    impl Sealed for super::A {}
    impl Sealed for super::B {}
}

/// A FlexSPI pin
pub trait Pin: super::Iomuxc {
    /// Alternate value for this pin
    const ALT: u32;
    /// Daisy register
    const DAISY: Option<super::Daisy>;
    /// FlexSPI signal
    type Signal: Signal;
    /// FlexSPI port
    type Port: Port;
    /// FlexSPI module; `U1` for `FlexSPI1`
    type Module: super::consts::Unsigned;
}

/// Prepare a FlexSpi pin.
pub fn prepare<P: Pin>(pin: &mut P) {
    super::alternate(pin, P::ALT);
    super::set_sion(pin);
    if let Some(daisy) = P::DAISY {
        unsafe { daisy.write() };
    }
}

#[allow(unused)] // Used in chip-specific modules...
macro_rules! flexspi {
    (module: $module:ty, port: $port:ty, signal: $signal:ty, pad: $pad:ty, alt: $alt:expr, daisy: $daisy:expr) => {
        impl Pin for $pad {
            const ALT: u32 = $alt;
            const DAISY: Option<Daisy> = $daisy;
            type Signal = $signal;
            type Module = $module;
            type Port = $port;
        }
    };
}
