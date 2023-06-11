//! FlexSPI pin implementation

use super::pads::{
    gpio_ad_b1::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*, gpio_spi_b0::*, gpio_spi_b1::*,
};

use crate::{
    consts::*,
    flexspi::{Data0, Data1, Data2, Data3, Dqs, Pin, Sclk, Ss0b, Ss1b, A, B},
    Daisy,
};

//
// FlexSPI1 A
//
flexspi!(module: U1, port: A, signal: Data0, pad: GPIO_AD_B1_13, alt: 0, daisy: Some(DAISY_FLEXSPIA_DATA0_GPIO_AD_B1_13));
flexspi!(module: U1, port: A, signal: Data0, pad: GPIO_SD_B1_08, alt: 1, daisy: Some(DAISY_FLEXSPIA_DATA0_GPIO_SD_B1_08));
flexspi!(module: U1, port: A, signal: Data1, pad: GPIO_AD_B1_12, alt: 0, daisy: Some(DAISY_FLEXSPIA_DATA1_GPIO_AD_B1_12));
flexspi!(module: U1, port: A, signal: Data1, pad: GPIO_SD_B1_09, alt: 1, daisy: Some(DAISY_FLEXSPIA_DATA1_GPIO_SD_B1_09));
flexspi!(module: U1, port: A, signal: Data2, pad: GPIO_AD_B1_11, alt: 0, daisy: Some(DAISY_FLEXSPIA_DATA2_GPIO_AD_B1_11));
flexspi!(module: U1, port: A, signal: Data2, pad: GPIO_SD_B1_10, alt: 1, daisy: Some(DAISY_FLEXSPIA_DATA2_GPIO_SD_B1_10));
flexspi!(module: U1, port: A, signal: Data3, pad: GPIO_AD_B1_10, alt: 0, daisy: Some(DAISY_FLEXSPIA_DATA3_GPIO_AD_B1_10));
flexspi!(module: U1, port: A, signal: Data3, pad: GPIO_SD_B1_11, alt: 1, daisy: Some(DAISY_FLEXSPIA_DATA3_GPIO_SD_B1_11));
flexspi!(module: U1, port: A, signal: Dqs, pad: GPIO_AD_B1_09, alt: 0, daisy: Some(DAISY_FLEXSPIA_DQS_GPIO_AD_B1_09));
flexspi!(module: U1, port: A, signal: Dqs, pad: GPIO_SD_B1_05, alt: 1, daisy: Some(DAISY_FLEXSPIA_DQS_GPIO_SD_B1_05));
flexspi!(module: U1, port: A, signal: Sclk, pad: GPIO_AD_B1_14, alt: 0, daisy: Some(DAISY_FLEXSPIA_SCK_GPIO_AD_B1_14));
flexspi!(module: U1, port: A, signal: Sclk, pad: GPIO_SD_B1_07, alt: 1, daisy: Some(DAISY_FLEXSPIA_SCK_GPIO_SD_B1_07));
flexspi!(module: U1, port: A, signal: Ss0b, pad: GPIO_AD_B1_15, alt: 0, daisy: None);
flexspi!(module: U1, port: A, signal: Ss0b, pad: GPIO_SD_B1_06, alt: 1, daisy: None);
flexspi!(module: U1, port: A, signal: Ss1b, pad: GPIO_AD_B1_08, alt: 0, daisy: None);
flexspi!(module: U1, port: A, signal: Ss1b, pad: GPIO_SD_B0_00, alt: 6, daisy: None);
// TODO: figure out how to handle pins with multiple functions
// Maybe via feature flag?
//flexspi!(module: U1, port: A, signal: Ss1b, pad: GPIO_SD_B1_04, alt: 4, daisy: None);

//
// FlexSPI1 B
//
flexspi!(module: U1, port: B, signal: Data0, pad: GPIO_AD_B1_07, alt: 0, daisy: Some(DAISY_FLEXSPIB_DATA0_GPIO_AD_B1_07));
flexspi!(module: U1, port: B, signal: Data0, pad: GPIO_SD_B1_03, alt: 1, daisy: Some(DAISY_FLEXSPIB_DATA0_GPIO_SD_B1_03));
flexspi!(module: U1, port: B, signal: Data1, pad: GPIO_AD_B1_06, alt: 0, daisy: Some(DAISY_FLEXSPIB_DATA1_GPIO_AD_B1_06));
flexspi!(module: U1, port: B, signal: Data1, pad: GPIO_SD_B1_02, alt: 1, daisy: Some(DAISY_FLEXSPIB_DATA1_GPIO_SD_B1_02));
flexspi!(module: U1, port: B, signal: Data2, pad: GPIO_AD_B1_05, alt: 0, daisy: Some(DAISY_FLEXSPIB_DATA2_GPIO_AD_B1_05));
flexspi!(module: U1, port: B, signal: Data2, pad: GPIO_SD_B1_01, alt: 1, daisy: Some(DAISY_FLEXSPIB_DATA2_GPIO_SD_B1_01));
flexspi!(module: U1, port: B, signal: Data3, pad: GPIO_AD_B1_04, alt: 0, daisy: Some(DAISY_FLEXSPIB_DATA3_GPIO_AD_B1_04));
flexspi!(module: U1, port: B, signal: Data3, pad: GPIO_SD_B1_00, alt: 1, daisy: Some(DAISY_FLEXSPIB_DATA3_GPIO_SD_B1_00));
flexspi!(module: U1, port: B, signal: Dqs, pad: GPIO_SD_B0_05, alt: 4, daisy: None);
flexspi!(module: U1, port: B, signal: Sclk, pad: GPIO_SD_B1_04, alt: 1, daisy: None);
flexspi!(module: U1, port: B, signal: Ss0b, pad: GPIO_SD_B0_04, alt: 4, daisy: None);
//flexspi!(module: U1, port: B, signal: Ss0b, pad: GPIO_SD_B1_05, alt: 4, daisy: None);
flexspi!(module: U1, port: B, signal: Ss1b, pad: GPIO_SD_B0_01, alt: 6, daisy: None);

//
// FlexSPI2 A
//
flexspi!(module: U2, port: A, signal: Data0, pad: GPIO_EMC_26, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT0_GPIO_EMC_26));
flexspi!(module: U2, port: A, signal: Data0, pad: GPIO_SPI_B0_02, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT0_GPIO_SPI_B0_02));
flexspi!(module: U2, port: A, signal: Data0, pad: GPIO_SPI_B1_04, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT0_GPIO_SPI_B1_04));
flexspi!(module: U2, port: A, signal: Data1, pad: GPIO_EMC_27, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT1_GPIO_EMC_27));
flexspi!(module: U2, port: A, signal: Data1, pad: GPIO_SPI_B0_12, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT1_GPIO_SPI_B0_12));
flexspi!(module: U2, port: A, signal: Data1, pad: GPIO_SPI_B1_03, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT1_GPIO_SPI_B1_03));
flexspi!(module: U2, port: A, signal: Data2, pad: GPIO_EMC_28, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT2_GPIO_EMC_28));
flexspi!(module: U2, port: A, signal: Data2, pad: GPIO_SPI_B0_06, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT2_GPIO_SPI_B0_06));
flexspi!(module: U2, port: A, signal: Data2, pad: GPIO_SPI_B1_02, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT2_GPIO_SPI_B1_02));
flexspi!(module: U2, port: A, signal: Data3, pad: GPIO_EMC_29, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT3_GPIO_EMC_29));
flexspi!(module: U2, port: A, signal: Data3, pad: GPIO_SPI_B0_10, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT3_GPIO_SPI_B0_10));
flexspi!(module: U2, port: A, signal: Data3, pad: GPIO_SPI_B1_01, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT3_GPIO_SPI_B1_01));
flexspi!(module: U2, port: A, signal: Dqs, pad: GPIO_EMC_23, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_DQS_FA_GPIO_EMC_23));
flexspi!(module: U2, port: A, signal: Dqs, pad: GPIO_SPI_B0_09, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_DQS_FA_GPIO_SPI_B0_09));
flexspi!(module: U2, port: A, signal: Dqs, pad: GPIO_SPI_B1_00, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_DQS_FA_GPIO_SPI_B1_00));
flexspi!(module: U2, port: A, signal: Sclk, pad: GPIO_EMC_25, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_SCK_FA_GPIO_EMC_25));
flexspi!(module: U2, port: A, signal: Sclk, pad: GPIO_SPI_B0_08, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_SCK_FA_GPIO_SPI_B0_08));
flexspi!(module: U2, port: A, signal: Sclk, pad: GPIO_SPI_B1_05, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_SCK_FA_GPIO_SPI_B1_05));
flexspi!(module: U2, port: A, signal: Ss0b, pad: GPIO_EMC_24, alt: 8,    daisy: None);
flexspi!(module: U2, port: A, signal: Ss0b, pad: GPIO_SPI_B0_05, alt: 0, daisy: None);
flexspi!(module: U2, port: A, signal: Ss0b, pad: GPIO_SPI_B1_06, alt: 0, daisy: None);
flexspi!(module: U2, port: A, signal: Ss1b, pad: GPIO_EMC_22, alt: 8,    daisy: None);

//
// FlexSPI2 B
//
flexspi!(module: U2, port: B, signal: Data0, pad: GPIO_EMC_13, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT0_GPIO_EMC_13));
flexspi!(module: U2, port: B, signal: Data0, pad: GPIO_SPI_B0_11, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT0_GPIO_SPI_B0_11));
flexspi!(module: U2, port: B, signal: Data1, pad: GPIO_EMC_14, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT1_GPIO_EMC_14));
flexspi!(module: U2, port: B, signal: Data1, pad: GPIO_SPI_B0_07, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT1_GPIO_SPI_B0_07));
flexspi!(module: U2, port: B, signal: Data2, pad: GPIO_EMC_15, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT2_GPIO_EMC_15));
flexspi!(module: U2, port: B, signal: Data2, pad: GPIO_SPI_B0_03, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT2_GPIO_SPI_B0_03));
flexspi!(module: U2, port: B, signal: Data3, pad: GPIO_EMC_16, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT3_GPIO_EMC_16));
flexspi!(module: U2, port: B, signal: Data3, pad: GPIO_SPI_B0_04, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT3_GPIO_SPI_B0_04));
flexspi!(module: U2, port: B, signal: Dqs, pad: GPIO_EMC_11, alt: 8, daisy: None);
flexspi!(module: U2, port: B, signal: Sclk, pad: GPIO_EMC_12, alt: 8, daisy: Some(DAISY_FLEXSPI2_IPP_IND_SCK_FB_GPIO_EMC_12));
flexspi!(module: U2, port: B, signal: Sclk, pad: GPIO_SPI_B0_01, alt: 0, daisy: Some(DAISY_FLEXSPI2_IPP_IND_SCK_FB_GPIO_SPI_B0_01));
flexspi!(module: U2, port: B, signal: Ss0b, pad: GPIO_EMC_10, alt: 8, daisy: None);
flexspi!(module: U2, port: B, signal: Ss1b, pad: GPIO_EMC_09, alt: 8, daisy: None);

/// Auto-generated DAISY values
mod daisy {
    use super::Daisy;

    pub const DAISY_FLEXSPIA_DQS_GPIO_SD_B1_05: Daisy = Daisy::new(0x401f84a4 as *mut u32, 0);
    pub const DAISY_FLEXSPIA_DQS_GPIO_AD_B1_09: Daisy = Daisy::new(0x401f84a4 as *mut u32, 1);
    pub const DAISY_FLEXSPIA_DATA0_GPIO_SD_B1_08: Daisy = Daisy::new(0x401f84a8 as *mut u32, 0);
    pub const DAISY_FLEXSPIA_DATA0_GPIO_AD_B1_13: Daisy = Daisy::new(0x401f84a8 as *mut u32, 1);
    pub const DAISY_FLEXSPIA_DATA1_GPIO_SD_B1_09: Daisy = Daisy::new(0x401f84ac as *mut u32, 0);
    pub const DAISY_FLEXSPIA_DATA1_GPIO_AD_B1_12: Daisy = Daisy::new(0x401f84ac as *mut u32, 1);
    pub const DAISY_FLEXSPIA_DATA2_GPIO_SD_B1_10: Daisy = Daisy::new(0x401f84b0 as *mut u32, 0);
    pub const DAISY_FLEXSPIA_DATA2_GPIO_AD_B1_11: Daisy = Daisy::new(0x401f84b0 as *mut u32, 1);
    pub const DAISY_FLEXSPIA_DATA3_GPIO_SD_B1_11: Daisy = Daisy::new(0x401f84b4 as *mut u32, 0);
    pub const DAISY_FLEXSPIA_DATA3_GPIO_AD_B1_10: Daisy = Daisy::new(0x401f84b4 as *mut u32, 1);
    pub const DAISY_FLEXSPIB_DATA0_GPIO_SD_B1_03: Daisy = Daisy::new(0x401f84b8 as *mut u32, 0);
    pub const DAISY_FLEXSPIB_DATA0_GPIO_AD_B1_07: Daisy = Daisy::new(0x401f84b8 as *mut u32, 1);
    pub const DAISY_FLEXSPIB_DATA1_GPIO_SD_B1_02: Daisy = Daisy::new(0x401f84bc as *mut u32, 0);
    pub const DAISY_FLEXSPIB_DATA1_GPIO_AD_B1_06: Daisy = Daisy::new(0x401f84bc as *mut u32, 1);
    pub const DAISY_FLEXSPIB_DATA2_GPIO_SD_B1_01: Daisy = Daisy::new(0x401f84c0 as *mut u32, 0);
    pub const DAISY_FLEXSPIB_DATA2_GPIO_AD_B1_05: Daisy = Daisy::new(0x401f84c0 as *mut u32, 1);
    pub const DAISY_FLEXSPIB_DATA3_GPIO_SD_B1_00: Daisy = Daisy::new(0x401f84c4 as *mut u32, 0);
    pub const DAISY_FLEXSPIB_DATA3_GPIO_AD_B1_04: Daisy = Daisy::new(0x401f84c4 as *mut u32, 1);
    pub const DAISY_FLEXSPIA_SCK_GPIO_SD_B1_07: Daisy = Daisy::new(0x401f84c8 as *mut u32, 0);
    pub const DAISY_FLEXSPIA_SCK_GPIO_AD_B1_14: Daisy = Daisy::new(0x401f84c8 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_DQS_FA_GPIO_SPI_B1_00: Daisy =
        Daisy::new(0x401f872c as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_DQS_FA_GPIO_EMC_23: Daisy =
        Daisy::new(0x401f872c as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_DQS_FA_GPIO_SPI_B0_09: Daisy =
        Daisy::new(0x401f872c as *mut u32, 2);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT0_GPIO_SPI_B1_04: Daisy =
        Daisy::new(0x401f8730 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT0_GPIO_EMC_26: Daisy =
        Daisy::new(0x401f8730 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT0_GPIO_SPI_B0_02: Daisy =
        Daisy::new(0x401f8730 as *mut u32, 2);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT1_GPIO_SPI_B1_03: Daisy =
        Daisy::new(0x401f8734 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT1_GPIO_EMC_27: Daisy =
        Daisy::new(0x401f8734 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT1_GPIO_SPI_B0_12: Daisy =
        Daisy::new(0x401f8734 as *mut u32, 2);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT2_GPIO_SPI_B1_02: Daisy =
        Daisy::new(0x401f8738 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT2_GPIO_EMC_28: Daisy =
        Daisy::new(0x401f8738 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT2_GPIO_SPI_B0_06: Daisy =
        Daisy::new(0x401f8738 as *mut u32, 2);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT3_GPIO_SPI_B1_01: Daisy =
        Daisy::new(0x401f873c as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT3_GPIO_EMC_29: Daisy =
        Daisy::new(0x401f873c as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FA_BIT3_GPIO_SPI_B0_10: Daisy =
        Daisy::new(0x401f873c as *mut u32, 2);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT0_GPIO_EMC_13: Daisy =
        Daisy::new(0x401f8740 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT0_GPIO_SPI_B0_11: Daisy =
        Daisy::new(0x401f8740 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT1_GPIO_EMC_14: Daisy =
        Daisy::new(0x401f8744 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT1_GPIO_SPI_B0_07: Daisy =
        Daisy::new(0x401f8744 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT2_GPIO_EMC_15: Daisy =
        Daisy::new(0x401f8748 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT2_GPIO_SPI_B0_03: Daisy =
        Daisy::new(0x401f8748 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT3_GPIO_EMC_16: Daisy =
        Daisy::new(0x401f874c as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_IO_FB_BIT3_GPIO_SPI_B0_04: Daisy =
        Daisy::new(0x401f874c as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_SCK_FA_GPIO_SPI_B1_05: Daisy =
        Daisy::new(0x401f8750 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_SCK_FA_GPIO_EMC_25: Daisy =
        Daisy::new(0x401f8750 as *mut u32, 1);
    pub const DAISY_FLEXSPI2_IPP_IND_SCK_FA_GPIO_SPI_B0_08: Daisy =
        Daisy::new(0x401f8750 as *mut u32, 2);
    pub const DAISY_FLEXSPI2_IPP_IND_SCK_FB_GPIO_EMC_12: Daisy =
        Daisy::new(0x401f8754 as *mut u32, 0);
    pub const DAISY_FLEXSPI2_IPP_IND_SCK_FB_GPIO_SPI_B0_01: Daisy =
        Daisy::new(0x401f8754 as *mut u32, 1);
}

use daisy::*;
