# 0.3
- Remove `consts` module, make all constants associated constants on `EtherType`.
- Remove `CDP` constant.
# 0.2
- Add `EtherType::ieee_organization_address` function.
- Optimize generated code size.
# 0.1
- Add `std` and `serde` feature.
- Implement `fmt::Debug` and `FromStr` on `EtherType`.
- Implement `serde::Serialize` and `serde::Deserialize` on `EtherType`.
- Add `error` module with the `ParseEtherTypeError` enum.
