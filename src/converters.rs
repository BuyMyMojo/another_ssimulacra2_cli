use ssimulacra2::{ColorPrimaries, TransferCharacteristic};

use crate::enums::{ColourSpace, ColourTransferCharacteristic};

pub const fn colour_space_to_color_primaries(cs: &ColourSpace) -> ColorPrimaries {
    match cs {
        ColourSpace::BT470M => ColorPrimaries::BT470M,
        ColourSpace::BT470BG => ColorPrimaries::BT470BG,
        ColourSpace::ST170M => ColorPrimaries::ST170M,
        ColourSpace::ST240M => ColorPrimaries::ST240M,
        ColourSpace::Film => ColorPrimaries::Film,
        ColourSpace::BT2020 => ColorPrimaries::BT2020,
        ColourSpace::ST428 => ColorPrimaries::ST428,
        ColourSpace::P3DCI => ColorPrimaries::P3DCI,
        ColourSpace::P3Display => ColorPrimaries::P3Display,
        ColourSpace::Tech3213 => ColorPrimaries::Tech3213,
        _ => ColorPrimaries::BT709,
    }
}

pub const fn colour_transfer_to_transfer_char(
    ct: &ColourTransferCharacteristic,
) -> TransferCharacteristic {
    match ct {
        ColourTransferCharacteristic::BT1886 => TransferCharacteristic::BT1886,
        ColourTransferCharacteristic::BT470M => TransferCharacteristic::BT470M,
        ColourTransferCharacteristic::BT470BG => TransferCharacteristic::BT470BG,
        ColourTransferCharacteristic::ST170M => TransferCharacteristic::ST170M,
        ColourTransferCharacteristic::ST240M => TransferCharacteristic::ST240M,
        ColourTransferCharacteristic::Linear => TransferCharacteristic::Linear,
        ColourTransferCharacteristic::Logarithmic100 => TransferCharacteristic::Logarithmic100,
        ColourTransferCharacteristic::Logarithmic316 => TransferCharacteristic::Logarithmic316,
        ColourTransferCharacteristic::XVYCC => TransferCharacteristic::XVYCC,
        ColourTransferCharacteristic::BT1361E => TransferCharacteristic::BT1361E,
        ColourTransferCharacteristic::BT2020Ten => TransferCharacteristic::BT2020Ten,
        ColourTransferCharacteristic::BT2020Twelve => TransferCharacteristic::BT2020Twelve,
        ColourTransferCharacteristic::PerceptualQuantizer => {
            TransferCharacteristic::PerceptualQuantizer
        }
        ColourTransferCharacteristic::ST428 => TransferCharacteristic::ST428,
        ColourTransferCharacteristic::HybridLogGamma => TransferCharacteristic::HybridLogGamma,
        _ => TransferCharacteristic::SRGB,
    }
}
