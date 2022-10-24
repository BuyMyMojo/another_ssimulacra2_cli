# Another ssimulacra2 cli

[![Building](https://github.com/BuyMyMojo/ssimulacra2_bin/actions/workflows/rust.yml/badge.svg)](https://github.com/BuyMyMojo/ssimulacra2_bin/actions/workflows/rust.yml)

A fork of ssimulacra2_bin that I could not stop adding things to. *whoops*

<!-- [![Crates.io](https://img.shields.io/crates/v/ssimulacra2_rs?style=for-the-badge)](https://crates.io/crates/ssimulacra2_rs) -->
<!-- [![LICENSE](https://img.shields.io/crates/l/ssimulacra2_rs?style=for-the-badge)](https://github.com/rust-av/ssimulacra2_bin/blob/main/LICENSE) -->

Binary interface to the Rust implementation of the SSIMULACRA2 metric: <https://github.com/rust-av/ssimulacra2>

## usage

```bash
Another ssimulacra2 cli

Usage: as2c [OPTIONS] <SOURCE> <DISTORTED> [OUT]

Arguments:
  <SOURCE>     Original unmodified image
  <DISTORTED>  Distorted image
  [OUT]        Output folder or `.csv` file. Requires --folders

Options:
  -f, --folders
          If input paths are folders, process all images in the folders. This assumes the files are named the same in both folders.
      --colour-space <COLOUR_SPACE>
          https://docs.rs/av-data/0.4.1/av_data/pixel/enum.ColorPrimaries.html for more info [default: bt709] [possible values: reserved0, bt709, unspecified, reserved, bt470m, bt470bg, st170m, st240m, film, bt2020, st428, p3dci, p3-display, tech3213]
      --colour-transfer <COLOUR_TRANSFER>
          https://docs.rs/av-data/0.4.1/av_data/pixel/enum.TransferCharacteristic.html for more info [default: srgb] [possible values: reserved0, bt1886, unspecified, reserved, bt470m, bt470bg, st170m, st240m, linear, logarithmic100, logarithmic316, xvycc, bt1361e, srgb, bt2020-ten, bt2020-twelve, perceptual-quantizer, st428, hybrid-log-gamma]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

supported ColorPrimaries(`--colour-space`):

```txt
Reserved0
BT709
Unspecified
Reserved
BT470M
BT470BG
ST170M
ST240M
Film
BT2020
ST428
P3DCI
P3Display
Tech3213
```

supported TransferCharacteristic(`--colour-transfer`):

```txt
Reserved0
BT1886
Unspecified
Reserved
BT470M
BT470BG
ST170M
ST240M
Linear
Logarithmic100
Logarithmic316
XVYCC
BT1361E
SRGB
BT2020Ten
BT2020Twelve
PerceptualQuantizer
ST428
HybridLogGamma
```
