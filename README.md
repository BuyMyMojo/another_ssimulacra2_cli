# ssimulacra2_bin

<!-- [![Crates.io](https://img.shields.io/crates/v/ssimulacra2_rs?style=for-the-badge)](https://crates.io/crates/ssimulacra2_rs) -->
[![LICENSE](https://img.shields.io/crates/l/ssimulacra2_rs?style=for-the-badge)](https://github.com/rust-av/ssimulacra2_bin/blob/main/LICENSE)

Binary interface to the Rust implementation of the SSIMULACRA2 metric: <https://github.com/rust-av/ssimulacra2>

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