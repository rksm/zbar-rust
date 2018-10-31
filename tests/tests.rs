extern crate zbar_rust;
extern crate qrcode_generator;

use zbar_rust::{ZBarConfig, ZBarSymbolType, ZBarImageScanner};

use qrcode_generator::QrCodeEcc;

#[test]
fn decode_qrcode() {
    let mut scanner = ZBarImageScanner::new();
    scanner.set_config(ZBarSymbolType::ZBarNone, ZBarConfig::ZBarCfgEnable, 0).unwrap();
    scanner.set_config(ZBarSymbolType::ZBarQRCode, ZBarConfig::ZBarCfgEnable, 1).unwrap();

    let url = "https://magiclen.org";

    let size = 512;

    let data = qrcode_generator::to_image(url, QrCodeEcc::Low, size).unwrap();

    let mut result = scanner.scan_y800(&data, size as u32, size as u32).unwrap();

    assert_eq!(1, result.len());
    assert_eq!(ZBarSymbolType::ZBarQRCode, result[0].symbol_type);
    assert_eq!(url, unsafe { String::from_utf8_unchecked(result.remove(0).data) });
}