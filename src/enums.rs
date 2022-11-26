use clap::ValueEnum;

/// <https://docs.rs/av-data/0.4.1/av_data/pixel/enum.ColorPrimaries.html> for more info
#[derive(ValueEnum, Clone, Debug)]
pub enum ColourSpace {
    Reserved0,
    BT709,
    Unspecified,
    Reserved,
    BT470M,
    BT470BG,
    ST170M,
    ST240M,
    Film,
    BT2020,
    ST428,
    P3DCI,
    P3Display,
    Tech3213,
}

/// <https://docs.rs/av-data/0.4.1/av_data/pixel/enum.TransferCharacteristic.html> for more info
#[derive(ValueEnum, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum ColourTransferCharacteristic {
    Reserved0,
    BT1886,
    Unspecified,
    Reserved,
    BT470M,
    BT470BG,
    ST170M,
    ST240M,
    Linear,
    Logarithmic100,
    Logarithmic316,
    XVYCC,
    BT1361E,
    SRGB,
    BT2020Ten,
    BT2020Twelve,
    PerceptualQuantizer,
    ST428,
    HybridLogGamma,
}
