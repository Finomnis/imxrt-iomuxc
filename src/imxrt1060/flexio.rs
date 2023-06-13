//! FlexSPI pin implementation

use super::pads::{gpio_ad_b1::*, gpio_b0::*, gpio_b1::*, gpio_emc::*};

use crate::{consts::*, flexio::Pin, Daisy};

//
// FlexIO 1
//
flexio!(module: 1, pin: U0, pad: GPIO_EMC_00, alt: 4, daisy: None);
flexio!(module: 1, pin: U1, pad: GPIO_EMC_01, alt: 4, daisy: None);
flexio!(module: 1, pin: U2, pad: GPIO_EMC_02, alt: 4, daisy: None);
flexio!(module: 1, pin: U3, pad: GPIO_EMC_03, alt: 4, daisy: None);
flexio!(module: 1, pin: U4, pad: GPIO_EMC_04, alt: 4, daisy: None);
flexio!(module: 1, pin: U5, pad: GPIO_EMC_05, alt: 4, daisy: None);
flexio!(module: 1, pin: U6, pad: GPIO_EMC_06, alt: 4, daisy: None);
flexio!(module: 1, pin: U7, pad: GPIO_EMC_07, alt: 4, daisy: None);
flexio!(module: 1, pin: U8, pad: GPIO_EMC_08, alt: 4, daisy: None);
flexio!(module: 1, pin: U9, pad: GPIO_EMC_09, alt: 4, daisy: None);
flexio!(module: 1, pin: U10, pad: GPIO_EMC_10, alt: 4, daisy: None);
flexio!(module: 1, pin: U11, pad: GPIO_EMC_11, alt: 4, daisy: None);
flexio!(module: 1, pin: U12, pad: GPIO_EMC_26, alt: 4, daisy: None);
flexio!(module: 1, pin: U13, pad: GPIO_EMC_27, alt: 4, daisy: None);
flexio!(module: 1, pin: U14, pad: GPIO_EMC_28, alt: 4, daisy: None);
flexio!(module: 1, pin: U15, pad: GPIO_EMC_29, alt: 4, daisy: None);

//
// FlexIO 2
//
flexio!(module: 2, pin: U0, pad: GPIO_B0_00, alt: 4, daisy: None);
flexio!(module: 2, pin: U1, pad: GPIO_B0_01, alt: 4, daisy: None);
flexio!(module: 2, pin: U2, pad: GPIO_B0_02, alt: 4, daisy: None);
flexio!(module: 2, pin: U3, pad: GPIO_B0_03, alt: 4, daisy: None);
flexio!(module: 2, pin: U4, pad: GPIO_B0_04, alt: 4, daisy: None);
flexio!(module: 2, pin: U5, pad: GPIO_B0_05, alt: 4, daisy: None);
flexio!(module: 2, pin: U6, pad: GPIO_B0_06, alt: 4, daisy: None);
flexio!(module: 2, pin: U7, pad: GPIO_B0_07, alt: 4, daisy: None);
flexio!(module: 2, pin: U8, pad: GPIO_B0_08, alt: 4, daisy: None);
flexio!(module: 2, pin: U9, pad: GPIO_B0_09, alt: 4, daisy: None);
flexio!(module: 2, pin: U10, pad: GPIO_B0_10, alt: 4, daisy: None);
flexio!(module: 2, pin: U11, pad: GPIO_B0_11, alt: 4, daisy: None);
flexio!(module: 2, pin: U12, pad: GPIO_B0_12, alt: 4, daisy: None);
flexio!(module: 2, pin: U13, pad: GPIO_B0_13, alt: 4, daisy: None);
flexio!(module: 2, pin: U14, pad: GPIO_B0_14, alt: 4, daisy: None);
flexio!(module: 2, pin: U15, pad: GPIO_B0_15, alt: 4, daisy: None);
flexio!(module: 2, pin: U16, pad: GPIO_B1_00, alt: 4, daisy: None);
flexio!(module: 2, pin: U17, pad: GPIO_B1_01, alt: 4, daisy: None);
flexio!(module: 2, pin: U18, pad: GPIO_B1_02, alt: 4, daisy: None);
flexio!(module: 2, pin: U19, pad: GPIO_B1_03, alt: 4, daisy: None);
flexio!(module: 2, pin: U20, pad: GPIO_B1_04, alt: 4, daisy: None);
flexio!(module: 2, pin: U21, pad: GPIO_B1_05, alt: 4, daisy: None);
flexio!(module: 2, pin: U22, pad: GPIO_B1_06, alt: 4, daisy: None);
flexio!(module: 2, pin: U23, pad: GPIO_B1_07, alt: 4, daisy: None);
flexio!(module: 2, pin: U24, pad: GPIO_B1_08, alt: 4, daisy: None);
flexio!(module: 2, pin: U25, pad: GPIO_B1_09, alt: 4, daisy: None);
flexio!(module: 2, pin: U26, pad: GPIO_B1_10, alt: 4, daisy: None);
flexio!(module: 2, pin: U27, pad: GPIO_B1_11, alt: 4, daisy: None);
flexio!(module: 2, pin: U28, pad: GPIO_B1_12, alt: 4, daisy: None);
flexio!(module: 2, pin: U29, pad: GPIO_B1_13, alt: 4, daisy: None);
flexio!(module: 2, pin: U30, pad: GPIO_B1_14, alt: 4, daisy: None);
flexio!(module: 2, pin: U31, pad: GPIO_B1_15, alt: 4, daisy: None);

//
// FlexIO 3
//
flexio!(module: 3, pin: U0, pad: GPIO_AD_B1_00, alt: 9, daisy: None);
flexio!(module: 3, pin: U1, pad: GPIO_AD_B1_01, alt: 9, daisy: None);
flexio!(module: 3, pin: U2, pad: GPIO_AD_B1_02, alt: 9, daisy: None);
flexio!(module: 3, pin: U3, pad: GPIO_AD_B1_03, alt: 9, daisy: None);
flexio!(module: 3, pin: U4, pad: GPIO_AD_B1_04, alt: 9, daisy: None);
flexio!(module: 3, pin: U5, pad: GPIO_AD_B1_05, alt: 9, daisy: None);
flexio!(module: 3, pin: U6, pad: GPIO_AD_B1_06, alt: 9, daisy: None);
flexio!(module: 3, pin: U7, pad: GPIO_AD_B1_07, alt: 9, daisy: None);
flexio!(module: 3, pin: U8, pad: GPIO_AD_B1_08, alt: 9, daisy: None);
flexio!(module: 3, pin: U9, pad: GPIO_AD_B1_09, alt: 9, daisy: None);
flexio!(module: 3, pin: U10, pad: GPIO_AD_B1_10, alt: 9, daisy: None);
flexio!(module: 3, pin: U11, pad: GPIO_AD_B1_11, alt: 9, daisy: None);
flexio!(module: 3, pin: U12, pad: GPIO_AD_B1_12, alt: 9, daisy: None);
flexio!(module: 3, pin: U13, pad: GPIO_AD_B1_13, alt: 9, daisy: None);
flexio!(module: 3, pin: U14, pad: GPIO_AD_B1_14, alt: 9, daisy: None);
flexio!(module: 3, pin: U15, pad: GPIO_AD_B1_15, alt: 9, daisy: None);
flexio!(module: 3, pin: U16, pad: GPIO_B1_00, alt: 9, daisy: None);
flexio!(module: 3, pin: U17, pad: GPIO_B1_01, alt: 9, daisy: None);
flexio!(module: 3, pin: U18, pad: GPIO_B1_02, alt: 9, daisy: None);
flexio!(module: 3, pin: U19, pad: GPIO_B1_03, alt: 9, daisy: None);
flexio!(module: 3, pin: U20, pad: GPIO_B1_04, alt: 9, daisy: None);
flexio!(module: 3, pin: U21, pad: GPIO_B1_05, alt: 9, daisy: None);
flexio!(module: 3, pin: U22, pad: GPIO_B1_06, alt: 9, daisy: None);
flexio!(module: 3, pin: U23, pad: GPIO_B1_07, alt: 9, daisy: None);
flexio!(module: 3, pin: U24, pad: GPIO_B1_08, alt: 9, daisy: None);
flexio!(module: 3, pin: U25, pad: GPIO_B1_09, alt: 9, daisy: None);
flexio!(module: 3, pin: U26, pad: GPIO_B1_10, alt: 9, daisy: None);
flexio!(module: 3, pin: U27, pad: GPIO_B1_11, alt: 9, daisy: None);
flexio!(module: 3, pin: U28, pad: GPIO_B1_12, alt: 9, daisy: None);
flexio!(module: 3, pin: U29, pad: GPIO_B1_13, alt: 9, daisy: None);
flexio!(module: 3, pin: U30, pad: GPIO_B1_14, alt: 9, daisy: None);
flexio!(module: 3, pin: U31, pad: GPIO_B1_15, alt: 9, daisy: None);

// FlexIO on this chip does not have any daisy values.
