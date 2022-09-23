use rsa::{BigUint, RsaPrivateKey};

pub fn pkg_key3() -> RsaPrivateKey {
    let n = BigUint::from_bytes_be(&[
        0xd2, 0x12, 0xfc, 0x33, 0x5f, 0x6d, 0xdb, 0x83, 0x16, 0x09, 0x62, 0x8b, 0x03, 0x56, 0x27,
        0x37, 0x82, 0xd4, 0x77, 0x85, 0x35, 0x29, 0x39, 0x2d, 0x52, 0x6b, 0x8c, 0x4c, 0x8c, 0xfb,
        0x06, 0xc1, 0x84, 0x5b, 0xe7, 0xd4, 0xf7, 0xbc, 0xd2, 0x4e, 0x62, 0x45, 0xcd, 0x2a, 0xbb,
        0xd7, 0x77, 0x76, 0x45, 0x36, 0x55, 0x27, 0x3f, 0xb3, 0xf5, 0xf9, 0x8e, 0xda, 0x4b, 0xef,
        0xaa, 0x59, 0xae, 0xb3, 0x9b, 0xea, 0x54, 0x98, 0xd2, 0x06, 0x32, 0x6a, 0x58, 0x31, 0x2a,
        0xe0, 0xd4, 0x4f, 0x90, 0xb5, 0x0a, 0x7d, 0xec, 0xf4, 0x3a, 0x9c, 0x52, 0x67, 0x2d, 0x99,
        0x31, 0x8e, 0x0c, 0x43, 0xe6, 0x82, 0xfe, 0x07, 0x46, 0xe1, 0x2e, 0x50, 0xd4, 0x1f, 0x2d,
        0x2f, 0x7e, 0xd9, 0x08, 0xba, 0x06, 0xb3, 0xbf, 0x2e, 0x20, 0x3f, 0x4e, 0x3f, 0xfe, 0x44,
        0xff, 0xaa, 0x50, 0x43, 0x57, 0x91, 0x69, 0x94, 0x49, 0x15, 0x82, 0x82, 0xe4, 0x0f, 0x4c,
        0x8d, 0x9d, 0x2c, 0xc9, 0x5b, 0x1d, 0x64, 0xbf, 0x88, 0x8b, 0xd4, 0xc5, 0x94, 0xe7, 0x65,
        0x47, 0x84, 0x1e, 0xe5, 0x79, 0x10, 0xfb, 0x98, 0x93, 0x47, 0xb9, 0x7d, 0x85, 0x12, 0xa6,
        0x40, 0x98, 0x2c, 0xf7, 0x92, 0xbc, 0x95, 0x19, 0x32, 0xed, 0xe8, 0x90, 0x56, 0x0d, 0x65,
        0xc1, 0xaa, 0x78, 0xc6, 0x2e, 0x54, 0xfd, 0x5f, 0x54, 0xa1, 0xf6, 0x7e, 0xe5, 0xe0, 0x5f,
        0x61, 0xc1, 0x20, 0xb4, 0xb9, 0xb4, 0x33, 0x08, 0x70, 0xe4, 0xdf, 0x89, 0x56, 0xed, 0x01,
        0x29, 0x46, 0x77, 0x5f, 0x8c, 0xb8, 0xa9, 0xf5, 0x1e, 0x2e, 0xb3, 0xb9, 0xbf, 0xe0, 0x09,
        0xb7, 0x8d, 0x28, 0xd4, 0xa6, 0xc3, 0xb8, 0x1e, 0x1f, 0x07, 0xeb, 0xb4, 0x12, 0x0b, 0x95,
        0xb8, 0x85, 0x30, 0xfd, 0xdc, 0x39, 0x13, 0xd0, 0x7c, 0xdc, 0x8f, 0xed, 0xf9, 0xc9, 0xa3,
        0xc1,
    ]);

    let e = BigUint::from_bytes_be(&[0x00, 0x01, 0x00, 0x01]);

    let d = BigUint::from_bytes_be(&[
        0x32, 0xd9, 0x03, 0x90, 0x8f, 0xbd, 0xb0, 0x8f, 0x57, 0x2b, 0x28, 0x5e, 0x0b, 0x8d, 0xb3,
        0xea, 0x5c, 0xd1, 0x7e, 0xa8, 0x90, 0x88, 0x8c, 0xdd, 0x6a, 0x80, 0xbb, 0xb1, 0xdf, 0xc1,
        0xf7, 0x0d, 0xaa, 0x32, 0xf0, 0xb7, 0x7c, 0xcb, 0x88, 0x80, 0x0e, 0x8b, 0x64, 0xb0, 0xbe,
        0x4c, 0xd6, 0x0e, 0x9b, 0x8c, 0x1e, 0x2a, 0x64, 0xe1, 0xf3, 0x5c, 0xd7, 0x76, 0x01, 0x41,
        0x5e, 0x93, 0x5c, 0x94, 0xfe, 0xdd, 0x46, 0x62, 0xc3, 0x1b, 0x5a, 0xe2, 0xa0, 0xbc, 0x2d,
        0xeb, 0xc3, 0x98, 0x0a, 0xa7, 0xb7, 0x85, 0x69, 0x70, 0x68, 0x2b, 0x64, 0x4a, 0xb3, 0x1f,
        0xcc, 0x7d, 0xdc, 0x7c, 0x26, 0xf4, 0x77, 0xf6, 0x5c, 0xf2, 0xae, 0x5a, 0x44, 0x2d, 0xd3,
        0xab, 0x16, 0x62, 0x04, 0x19, 0xba, 0xfb, 0x90, 0xff, 0xe2, 0x30, 0x50, 0x89, 0x6e, 0xcb,
        0x56, 0xb2, 0xeb, 0xc0, 0x91, 0x16, 0x92, 0x5e, 0x30, 0x8e, 0xae, 0xc7, 0x94, 0x5d, 0xfd,
        0x35, 0xe1, 0x20, 0xf8, 0xad, 0x3e, 0xbc, 0x08, 0xbf, 0xc0, 0x36, 0x74, 0x9f, 0xd5, 0xbb,
        0x52, 0x08, 0xfd, 0x06, 0x66, 0xf3, 0x7a, 0xb3, 0x04, 0xf4, 0x75, 0x29, 0x5d, 0xe9, 0x5f,
        0xaa, 0x10, 0x30, 0xb2, 0x0f, 0x5a, 0x1a, 0xc1, 0x2a, 0xb3, 0xfe, 0xcb, 0x21, 0xad, 0x80,
        0xec, 0x8f, 0x20, 0x09, 0x1c, 0xdb, 0xc5, 0x58, 0x94, 0xc2, 0x9c, 0xc6, 0xce, 0x82, 0x65,
        0x3e, 0x57, 0x90, 0xbc, 0xa9, 0x8b, 0x06, 0xb4, 0xf0, 0x72, 0xf6, 0x77, 0xdf, 0x98, 0x64,
        0xf1, 0xec, 0xfe, 0x37, 0x2d, 0xbc, 0xae, 0x8c, 0x08, 0x81, 0x1f, 0xc3, 0xc9, 0x89, 0x1a,
        0xc7, 0x42, 0x82, 0x4b, 0x2e, 0xdc, 0x8e, 0x8d, 0x73, 0xce, 0xb1, 0xcc, 0x01, 0xd9, 0x08,
        0x70, 0x87, 0x3c, 0x44, 0x08, 0xec, 0x49, 0x8f, 0x81, 0x5a, 0xe2, 0x40, 0xff, 0x77, 0xfc,
        0x0d,
    ]);

    let primes = vec![
        BigUint::from_bytes_be(&[
            0xf9, 0x67, 0xad, 0x99, 0x12, 0x31, 0x0c, 0x56, 0xa2, 0x2e, 0x16, 0x1c, 0x46, 0xb3,
            0x4d, 0x5b, 0x43, 0xbe, 0x42, 0xa2, 0xf6, 0x86, 0x96, 0x80, 0x42, 0xc3, 0xc7, 0x3f,
            0xc3, 0x42, 0xf5, 0x87, 0x49, 0x33, 0x9f, 0x07, 0x5d, 0x6e, 0x2c, 0x04, 0xfd, 0xe3,
            0xe1, 0xb2, 0xae, 0x0a, 0x0c, 0xf0, 0xc7, 0xa6, 0x1c, 0xa1, 0x63, 0x50, 0xc8, 0x09,
            0x9c, 0x51, 0x24, 0x52, 0x6c, 0x5e, 0x5e, 0xbd, 0x1e, 0x27, 0x06, 0xbb, 0xbc, 0x9e,
            0x94, 0xe1, 0x35, 0xd4, 0x6d, 0xb3, 0xcb, 0x3c, 0x68, 0xdd, 0x68, 0xb3, 0xfe, 0x6c,
            0xcb, 0x8d, 0x82, 0x20, 0x76, 0x23, 0x63, 0xb7, 0xe9, 0x68, 0x10, 0x01, 0x4e, 0xdc,
            0xba, 0x27, 0x5d, 0x01, 0xc1, 0x2d, 0x80, 0x5e, 0x2b, 0xaf, 0x82, 0x6b, 0xd8, 0x84,
            0xb6, 0x10, 0x52, 0x86, 0xa7, 0x89, 0x8e, 0xae, 0x9a, 0xe2, 0x89, 0xc6, 0xf7, 0xd5,
            0x87, 0xfb,
        ]),
        BigUint::from_bytes_be(&[
            0xd7, 0xa1, 0x0f, 0x9a, 0x8b, 0xf2, 0xc9, 0x11, 0x95, 0x32, 0x9a, 0x8c, 0xf0, 0xd9,
            0x40, 0x47, 0xf5, 0x68, 0xa0, 0x0d, 0xbd, 0xc1, 0xfc, 0x43, 0x2f, 0x65, 0xf9, 0xc3,
            0x61, 0x0f, 0x25, 0x77, 0x54, 0xad, 0xd7, 0x58, 0xac, 0x84, 0x40, 0x60, 0x8d, 0x3f,
            0xf3, 0x65, 0x89, 0x75, 0xb5, 0xc6, 0x2c, 0x51, 0x1a, 0x2f, 0x1f, 0x22, 0xe4, 0x43,
            0x11, 0x54, 0xbe, 0xc9, 0xb4, 0xc7, 0xb5, 0x1b, 0x05, 0x0b, 0xbc, 0x56, 0x9a, 0xcd,
            0x4a, 0xd9, 0x73, 0x68, 0x5e, 0x5c, 0xfb, 0x92, 0xb7, 0x8b, 0x0d, 0xff, 0xf5, 0x07,
            0xca, 0xb4, 0xc8, 0x9b, 0x96, 0x3c, 0x07, 0x9e, 0x3e, 0x6b, 0x2a, 0x11, 0xf2, 0x8a,
            0xb1, 0x8a, 0xd7, 0x2e, 0x1b, 0xa5, 0x53, 0x24, 0x06, 0xed, 0x50, 0xb8, 0x90, 0x67,
            0xb1, 0xe2, 0x41, 0xc6, 0x92, 0x01, 0xee, 0x10, 0xf0, 0x61, 0xbb, 0xfb, 0xb2, 0x7d,
            0x4a, 0x73,
        ]),
    ];

    RsaPrivateKey::from_components(n, e, d, primes)
}