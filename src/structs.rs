use clap::{command, Parser};

use crate::enums::{ColourSpace, ColourTransferCharacteristic};

/// struct to hold frame number and ssimulacra2 value
#[derive(Debug, Clone)]
pub struct FrameResult {
    /// frame number
    pub frame: u32,
    /// ssimulacra2 measurement
    pub ssimulacra2: f64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Source image
    #[arg(help = "Original unmodified image", value_hint = clap::ValueHint::FilePath)]
    pub source: String,

    /// Distorted image
    #[arg(help = "Distorted image", value_hint = clap::ValueHint::FilePath)]
    pub distorted: String,

    /// Location to output a .csv file with the ssimumulacra2 values
    #[arg(help = "Output folder or `.csv` file. Requires --folders", value_hint = clap::ValueHint::FilePath, requires = "folders")]
    pub out: Option<String>,

    /// Choose how many cpu threads to use. defaults to your core count!
    #[arg(long, short)]
    pub threads: Option<usize>,

    // TODO: Change help text to something more useful
    /// If input paths are folders, process all images in the folders
    #[arg(
        short,
        long,
        help = "If input paths are folders, process all images in the folders. This assumes the files are named the same in both folders."
    )]
    pub folders: bool,

    /// <https://docs.rs/av-data/0.4.1/av_data/pixel/enum.ColorPrimaries.html> for more info
    #[arg(long, value_enum, default_value_t = ColourSpace::BT709)]
    pub colour_space: ColourSpace,

    /// <https://docs.rs/av-data/0.4.1/av_data/pixel/enum.TransferCharacteristic.html> for more info
    #[arg(long, value_enum, default_value_t = ColourTransferCharacteristic::SRGB)]
    pub colour_transfer: ColourTransferCharacteristic,
}
