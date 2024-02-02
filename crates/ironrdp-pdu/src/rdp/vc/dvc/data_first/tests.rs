use lazy_static::lazy_static;

use super::*;
use crate::rdp::vc::dvc::ClientPdu;
use crate::{encode_vec, PduErrorKind};

const DVC_TEST_CHANNEL_ID_U8: u32 = 0x03;
const DVC_TEST_DATA_LENGTH: u32 = 0x0000_0C7B;

const DVC_FULL_DATA_FIRST_BUFFER_SIZE: usize = 16;
const DVC_DATA_FIRST_PREFIX: [u8; 4] = [0x24, 0x03, 0x7b, 0x0c];
const DVC_DATA_FIRST_BUFFER: [u8; 12] = [0x71, 0x71, 0x71, 0x71, 0x71, 0x71, 0x71, 0x71, 0x71, 0x71, 0x71, 0x71];

const DVC_DATA_FIRST_WITH_INVALID_TOTAL_MESSAGE_SIZE_BUFFER_SIZE: usize = 0x06;
const DVC_DATA_FIRST_WITH_INVALID_TOTAL_MESSAGE_SIZE_BUFFER: [u8;
    DVC_DATA_FIRST_WITH_INVALID_TOTAL_MESSAGE_SIZE_BUFFER_SIZE] = [0x03, 0x03, 0x71, 0x71, 0x71, 0x71];

const DVC_INVALID_DATA_MESSAGE_BUFFER: [u8; PDU_WITH_DATA_MAX_SIZE] = [0x77; PDU_WITH_DATA_MAX_SIZE];

const DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_PREFIX: [u8; 4] = [0x24, 0x7, 0x39, 0x6];

const DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_BUFFER: [u8; 1593] = [
    0xe0, 0x24, 0xa9, 0xba, 0xe0, 0x68, 0xa9, 0xba, 0x8a, 0x73, 0x41, 0x25, 0x12, 0x12, 0x1c, 0x28, 0x3b, 0xa6, 0x34,
    0x8, 0x8, 0x7a, 0x38, 0x34, 0x2c, 0xe8, 0xf8, 0xd0, 0xef, 0x18, 0xc2, 0xc, 0x27, 0x1f, 0xb1, 0x83, 0x3c, 0x58,
    0x8a, 0x67, 0x1, 0x58, 0x9d, 0x50, 0x8b, 0x8c, 0x60, 0x31, 0x53, 0x55, 0x54, 0xd8, 0x51, 0x32, 0x23, 0x54, 0xd9,
    0xd1, 0x65, 0x54, 0xd6, 0xe0, 0xb6, 0x4b, 0x8b, 0xd1, 0x1a, 0x32, 0x6d, 0x10, 0xed, 0x21, 0x48, 0x8d, 0x1d, 0xa2,
    0x6a, 0x90, 0x14, 0x88, 0x30, 0x18, 0x8a, 0x11, 0x1a, 0x37, 0xc6, 0xa7, 0x32, 0x8b, 0xea, 0x32, 0x44, 0x78, 0xc9,
    0x4e, 0xdb, 0x47, 0x11, 0x53, 0xc6, 0x16, 0xc9, 0x72, 0x55, 0x11, 0x4e, 0x12, 0x55, 0x31, 0xd4, 0x4b, 0x16, 0x71,
    0x7e, 0x32, 0x13, 0x5, 0xb2, 0x5c, 0x9c, 0xa4, 0xab, 0x78, 0xc9, 0x20, 0x88, 0xfb, 0x18, 0xa6, 0xb, 0x44, 0xac,
    0x6, 0x23, 0x4b, 0x2d, 0x12, 0xa3, 0x1a, 0x8b, 0xea, 0x2c, 0x44, 0x69, 0x12, 0xa8, 0xca, 0xa2, 0xc4, 0x64, 0x98,
    0x99, 0x11, 0xd4, 0x5e, 0x8a, 0x51, 0x92, 0x23, 0xa8, 0xbd, 0x34, 0x44, 0xc8, 0x65, 0x44, 0xd7, 0x30, 0x76, 0x11,
    0xea, 0x5a, 0xf7, 0x8b, 0xf1, 0x90, 0xaa, 0x90, 0x14, 0x52, 0x93, 0x4, 0x4c, 0x8b, 0x18, 0xc, 0x55, 0x2b, 0xd1,
    0x5a, 0x4e, 0xea, 0xa9, 0x82, 0x8b, 0xe0, 0xb2, 0x4a, 0x8e, 0x91, 0x62, 0x46, 0xf1, 0x8a, 0x77, 0x90, 0xd4, 0x89,
    0xa2, 0x34, 0x53, 0x15, 0x4f, 0xa4, 0xcf, 0x53, 0x2e, 0x93, 0x18, 0xa2, 0x35, 0x54, 0x51, 0x11, 0xa4, 0x91, 0x16,
    0x26, 0xd8, 0x44, 0xc9, 0x21, 0xde, 0x2f, 0xc6, 0x3b, 0x44, 0x69, 0xd, 0x44, 0xd0, 0x59, 0x99, 0xde, 0x4b, 0x33,
    0x44, 0x44, 0x9a, 0xa2, 0xe5, 0x55, 0xc0, 0x5b, 0x26, 0x55, 0x57, 0x92, 0x0, 0x96, 0xe5, 0x55, 0xb6, 0xa2, 0x6a,
    0x8b, 0xea, 0x3b, 0x45, 0xf5, 0x13, 0x44, 0x88, 0x4c, 0x85, 0x22, 0xc4, 0x46, 0x8e, 0xd1, 0x1a, 0x6d, 0x21, 0x68,
    0x97, 0x21, 0x88, 0xde, 0xa2, 0x94, 0x74, 0x95, 0xcc, 0x62, 0x9d, 0xdb, 0x2d, 0x66, 0x92, 0xc9, 0x5a, 0xa5, 0x90,
    0xa4, 0x46, 0x91, 0xb4, 0x85, 0x22, 0x94, 0x4d, 0x13, 0x37, 0xb, 0x84, 0xd9, 0x2a, 0x9a, 0xaf, 0x32, 0x3a, 0xa8,
    0xdd, 0x11, 0xa4, 0x87, 0x78, 0xbe, 0x91, 0xa8, 0x53, 0xcc, 0x95, 0x22, 0xca, 0x89, 0xb1, 0x8c, 0x92, 0x38, 0x91,
    0x34, 0x8f, 0xa6, 0x88, 0x4c, 0x71, 0x31, 0x8a, 0x50, 0x51, 0x1a, 0x34, 0x48, 0x4e, 0x31, 0x4e, 0x2b, 0x15, 0x62,
    0x96, 0x2c, 0xc6, 0x29, 0xe2, 0x46, 0x21, 0xf, 0x32, 0x32, 0xc0, 0x62, 0x49, 0x12, 0x6a, 0x88, 0xea, 0x2c, 0x46,
    0x88, 0x9a, 0xa4, 0xcd, 0x14, 0xa3, 0x84, 0x64, 0x91, 0xee, 0x28, 0xcb, 0x79, 0x4, 0x50, 0x44, 0x17, 0x9, 0xb1,
    0x42, 0x23, 0x47, 0xbb, 0xc5, 0xf5, 0x31, 0x2, 0x23, 0x45, 0x95, 0x13, 0x63, 0x27, 0xe8, 0xb3, 0x18, 0xa6, 0xa4,
    0x11, 0x31, 0xf6, 0x12, 0xc4, 0x8d, 0x2a, 0x26, 0xde, 0x63, 0x8e, 0x59, 0xd, 0xe5, 0x97, 0x49, 0x98, 0x99, 0x2e,
    0x9d, 0x12, 0x2b, 0xcb, 0x22, 0x9, 0x51, 0xde, 0x2f, 0x4b, 0xa6, 0x11, 0xea, 0x7e, 0xe4, 0x8c, 0x26, 0x1a, 0x49,
    0xb2, 0x23, 0xa8, 0xb1, 0x19, 0x24, 0x19, 0x11, 0xd4, 0x58, 0x8e, 0x2a, 0x43, 0x51, 0xda, 0x23, 0x46, 0x48, 0xee,
    0xa2, 0x66, 0x3, 0x11, 0x2a, 0x23, 0xeb, 0x90, 0xcd, 0x24, 0x1f, 0x1a, 0x16, 0x48, 0xb3, 0x19, 0x2a, 0xf2, 0x67,
    0xeb, 0x48, 0xaf, 0x14, 0x9f, 0xa4, 0xab, 0xd2, 0x6c, 0x35, 0x9a, 0x53, 0x7c, 0x24, 0xb7, 0x79, 0x8f, 0x91, 0x96,
    0xf2, 0x8, 0x9e, 0xc2, 0x65, 0x11, 0x58, 0xe8, 0x4c, 0xc9, 0x15, 0x8d, 0xc4, 0x47, 0x32, 0x18, 0x88, 0xea, 0x35,
    0x82, 0xd5, 0xab, 0xa, 0xe2, 0x46, 0xf7, 0x8b, 0xf7, 0x90, 0x75, 0x8, 0xe9, 0x8a, 0x11, 0x1d, 0x48, 0xf2, 0x4c,
    0xd3, 0x6d, 0xd4, 0x65, 0x51, 0x7a, 0x52, 0x14, 0x22, 0xd2, 0x37, 0xbc, 0x5f, 0xa4, 0x7c, 0x90, 0x74, 0x95, 0xb,
    0x64, 0xc9, 0x21, 0x44, 0x69, 0x1b, 0x8a, 0x13, 0x4a, 0x1, 0x28, 0x9a, 0x64, 0xc1, 0x1a, 0xc4, 0xc0, 0x6a, 0x28,
    0x68, 0x98, 0x5b, 0x26, 0x4c, 0x46, 0x92, 0x1d, 0x22, 0xc3, 0x61, 0xaa, 0xd2, 0x92, 0x44, 0x85, 0x22, 0x65, 0x3c,
    0xe8, 0xb2, 0xa9, 0x2a, 0x39, 0xa, 0x4a, 0x49, 0x3c, 0xba, 0x95, 0xe9, 0x92, 0xc6, 0x98, 0xd5, 0x81, 0x8f, 0x95,
    0x41, 0x94, 0x79, 0x93, 0x14, 0x8c, 0x71, 0x58, 0x27, 0xc9, 0x87, 0x73, 0xf1, 0x66, 0x3c, 0x7d, 0xd3, 0xe6, 0xac,
    0xa, 0xb7, 0x63, 0x18, 0x91, 0x8a, 0xc1, 0x48, 0xdc, 0xc9, 0x4c, 0x1e, 0xc, 0xec, 0x82, 0xd6, 0x97, 0x57, 0x90,
    0xfb, 0x89, 0x88, 0xdf, 0x70, 0xb0, 0xce, 0xae, 0x54, 0x1e, 0x4b, 0x7a, 0x7d, 0x88, 0x8c, 0xd, 0x24, 0x63, 0x43,
    0x7f, 0xc4, 0xee, 0xa3, 0x51, 0xb7, 0x27, 0xb9, 0x73, 0x4f, 0x91, 0xe3, 0xec, 0x79, 0xa9, 0xf9, 0xca, 0x89, 0x28,
    0x2d, 0x44, 0xe0, 0xf3, 0xd1, 0x77, 0x4b, 0xab, 0xc8, 0xb5, 0xc4, 0x74, 0xf, 0xf0, 0xff, 0x3b, 0x8d, 0xd2, 0xe,
    0x22, 0xf6, 0x8c, 0xc, 0xfd, 0xd3, 0x12, 0x31, 0xa9, 0x57, 0xf0, 0x50, 0xd6, 0x45, 0x30, 0xcb, 0x45, 0x82, 0xfe,
    0x9f, 0x63, 0xc3, 0x3, 0x32, 0xa1, 0xa1, 0x83, 0xf, 0xb5, 0x42, 0x85, 0x36, 0xf5, 0x4f, 0x18, 0x34, 0xf9, 0x1e,
    0x3e, 0xc7, 0x9a, 0xb9, 0xd9, 0xe5, 0x51, 0x7a, 0x8c, 0xac, 0x5b, 0x39, 0x98, 0x6e, 0x14, 0x87, 0x1c, 0x67, 0xc7,
    0xa3, 0xed, 0xde, 0x48, 0xd9, 0x47, 0xd0, 0xc5, 0x70, 0xa4, 0x38, 0xe3, 0x60, 0x1a, 0x17, 0x1e, 0x45, 0x1e, 0xdc,
    0x6c, 0x23, 0x1d, 0xc2, 0x90, 0xe3, 0x8d, 0x8c, 0x13, 0x8, 0xa5, 0x86, 0x9, 0x70, 0xca, 0x68, 0xc9, 0x70, 0xa4,
    0x38, 0xe3, 0x45, 0xa9, 0x5e, 0xce, 0x11, 0x23, 0xa5, 0x88, 0xed, 0x32, 0xd5, 0x83, 0xd3, 0x58, 0x31, 0x30, 0x46,
    0x8e, 0xc6, 0xc3, 0x88, 0xf4, 0xd2, 0x28, 0x7e, 0xfd, 0xff, 0xd2, 0xea, 0xf1, 0xf5, 0x71, 0x1d, 0xde, 0x65, 0xb6,
    0xf2, 0xa0, 0x98, 0xd2, 0x3c, 0x19, 0xf4, 0xba, 0xbc, 0x7d, 0x5c, 0x4b, 0x54, 0xab, 0x8d, 0xc0, 0xcc, 0xf0, 0xac,
    0x5d, 0x9a, 0x26, 0xae, 0x7c, 0x99, 0xd9, 0xf6, 0x70, 0x4c, 0xfe, 0xba, 0x7b, 0x70, 0x5f, 0x7f, 0x1a, 0xd7, 0x26,
    0x99, 0xb3, 0x13, 0x67, 0x54, 0xe7, 0x99, 0x22, 0x13, 0xae, 0x24, 0x63, 0x61, 0x52, 0xb5, 0x22, 0x1a, 0xa6, 0xac,
    0x7c, 0x90, 0x4, 0x8c, 0x68, 0xe5, 0x41, 0x36, 0x95, 0xa9, 0xc7, 0x8a, 0x7d, 0xea, 0x6b, 0xd2, 0xea, 0xf2, 0x81,
    0x71, 0x1f, 0x46, 0x4a, 0xe6, 0xb2, 0xa0, 0x64, 0x6d, 0x1a, 0x1b, 0x26, 0xcc, 0x4a, 0x82, 0x6b, 0x44, 0x9a, 0x64,
    0x61, 0x69, 0xa6, 0xd, 0x2e, 0x72, 0xab, 0x2d, 0x4a, 0x6d, 0x9b, 0x31, 0x2a, 0x8, 0xf3, 0x56, 0xc0, 0x7d, 0x9,
    0xfc, 0xb5, 0x15, 0x8c, 0x97, 0xe6, 0xed, 0x2e, 0xaf, 0x1f, 0x57, 0x10, 0xb7, 0x5, 0x80, 0x82, 0x23, 0x89, 0xbb,
    0xd9, 0xbf, 0x49, 0xf8, 0x95, 0x82, 0x74, 0xb6, 0x68, 0x25, 0xf, 0xb1, 0xd1, 0xe3, 0x27, 0x32, 0x6d, 0x4a, 0x70,
    0xd3, 0xee, 0x25, 0x41, 0x88, 0xf3, 0x47, 0x8b, 0x8d, 0x9f, 0xe0, 0x1a, 0x47, 0x39, 0x6, 0xce, 0x37, 0xa, 0x56,
    0xce, 0x34, 0x31, 0x71, 0x7e, 0xab, 0x37, 0x3a, 0xc2, 0x5b, 0xce, 0x5a, 0x5d, 0x5e, 0x40, 0x6e, 0x23, 0xf8, 0xd7,
    0xb3, 0xd5, 0xe6, 0xe0, 0x5e, 0xfa, 0x39, 0xcd, 0xd0, 0x95, 0x25, 0x4b, 0xdd, 0x31, 0x26, 0x41, 0x94, 0xcb, 0xd,
    0x3a, 0xd8, 0x12, 0x93, 0xda, 0x7, 0x4a, 0xa, 0x91, 0x41, 0xc0, 0xc0, 0xa9, 0xaa, 0x28, 0x2a, 0x60, 0x6c, 0xb,
    0x1e, 0x51, 0xe4, 0xc7, 0xaa, 0x35, 0x24, 0x75, 0xd2, 0xea, 0xf1, 0xe7, 0x71, 0x15, 0xdc, 0x4f, 0x12, 0x92, 0xad,
    0xf1, 0xbc, 0xd0, 0x76, 0xd2, 0xea, 0xf1, 0x5b, 0x71, 0x23, 0x93, 0xaa, 0xd5, 0xf1, 0xb1, 0xaa, 0x11, 0x1a, 0x9d,
    0xf4, 0xba, 0xbc, 0x48, 0xdc, 0x46, 0xf6, 0xc8, 0xa7, 0xe3, 0xdc, 0x88, 0xaf, 0x21, 0x9e, 0x25, 0xb2, 0x33, 0xf2,
    0xa9, 0xe2, 0x96, 0xcc, 0x9c, 0x2c, 0xd3, 0xca, 0xf4, 0x2e, 0xe6, 0x17, 0x25, 0xcf, 0x32, 0xd9, 0x19, 0xa1, 0xc0,
    0xac, 0x2f, 0x25, 0xb3, 0x5, 0xab, 0x87, 0xd1, 0x84, 0x4f, 0x91, 0x46, 0x4f, 0x5a, 0x5d, 0x5d, 0xf8, 0xae, 0x24,
    0x12, 0x4c, 0x58, 0xfa, 0x7d, 0xa9, 0x53, 0x8c, 0xe3, 0xdc, 0x8c, 0x3e, 0x48, 0xc2, 0xe1, 0x4b, 0xb9, 0xa6, 0xb,
    0x61, 0x27, 0x39, 0x10, 0xea, 0x93, 0x5c, 0x9f, 0x28, 0x54, 0x2c, 0xa4, 0xa2, 0x23, 0xcd, 0x3, 0xed, 0x4, 0x78,
    0x55, 0x60, 0xce, 0x35, 0x11, 0xcf, 0xb3, 0x9e, 0x46, 0x68, 0x64, 0x25, 0x43, 0x31, 0x11, 0x5c, 0xe3, 0xfc, 0x1c,
    0x36, 0xa2, 0x99, 0x87, 0xe9, 0xcf, 0x23, 0x34, 0x32, 0x11, 0xe6, 0x65, 0x17, 0x9b, 0x99, 0x17, 0xe1, 0xe4, 0x5e,
    0x24, 0x9f, 0xf4, 0xba, 0xbb, 0xca, 0xdc, 0x4c, 0xb7, 0xa1, 0x11, 0x4, 0xd6, 0x9, 0x6c, 0xdc, 0xfb, 0xa5, 0xd5,
    0xde, 0x8e, 0xe2, 0x15, 0xf0, 0x56, 0x95, 0x9c, 0x6, 0xb3, 0xa1, 0x69, 0x3, 0x32, 0xe2, 0x73, 0xe6, 0x26, 0xcc,
    0xd0, 0x88, 0x74, 0x2e, 0x6, 0x5, 0x2c, 0x42, 0xfb, 0x45, 0x5, 0x31, 0x24, 0x4d, 0xd0, 0xfb, 0x37, 0x46, 0x6c,
    0x47, 0xb9, 0xf, 0xcb, 0x35, 0x8d, 0xc1, 0x88, 0x83, 0xd2, 0xed, 0xd5, 0x94, 0xf1, 0x11, 0xdf, 0x2c, 0xb6, 0x29,
    0x39, 0xc6, 0x64, 0x2a, 0xa1, 0x24, 0x61, 0x89, 0x18, 0x7c, 0x91, 0x8c, 0xd3, 0x66, 0xc5, 0x8a, 0xa9, 0xc4, 0xd,
    0x3c, 0x33, 0x50, 0xba, 0x5d, 0xba, 0xad, 0x5e, 0x2d, 0xcf, 0x75, 0x32, 0x90, 0xee, 0xa8, 0xf4, 0xc5, 0x43, 0x50,
    0xa8, 0x62, 0x87, 0xd2, 0x46, 0x34, 0x43, 0xf4, 0x68, 0x88, 0x59, 0x3f, 0xa2, 0xd0, 0x7a, 0x1e, 0x69, 0x45, 0x56,
    0x3d, 0x33, 0x12, 0x70, 0x4c, 0x8d, 0x2c, 0x42, 0x1a, 0x94, 0xd4, 0xb7, 0x62, 0x18, 0x11, 0x1a, 0x5d, 0x5d, 0xe8,
    0xee, 0x23, 0x29, 0xc9, 0x8c, 0x65, 0x60, 0x7b, 0x7b, 0x8c, 0x11, 0x3a, 0x5d, 0xba, 0xcc, 0xde, 0x27, 0x99, 0x58,
    0x8a, 0x8c, 0x69, 0x91, 0xa, 0x2f, 0xb4, 0x55, 0xe, 0x4, 0xc1, 0xd, 0x25, 0x43, 0x42, 0xb2, 0x81, 0x5e, 0x56, 0x5f,
    0x16, 0xc6, 0x28, 0xb4, 0x5e, 0x97, 0x6e, 0xb3, 0x37, 0x89, 0x98, 0xe3, 0x13, 0xbb, 0xc9, 0x49, 0x87, 0xe1, 0x6d,
    0x19, 0xa5, 0xdb, 0xac, 0xcd, 0xe2, 0x32, 0xb3, 0x99, 0x2b, 0xf3, 0x60, 0xd1, 0x20, 0x7e, 0x46, 0xcc, 0x4, 0xc6,
    0x2c, 0xc0, 0x12, 0x26, 0x8, 0xc8, 0xad, 0xfa, 0xf6, 0x3f, 0x10, 0x45, 0xca, 0xe0, 0x9a, 0x3b, 0x4b, 0xb7, 0x57,
    0x33, 0xc4, 0x90, 0x72, 0xab, 0x5a, 0xcc, 0xf, 0x26, 0x42, 0xea, 0x3e, 0x87, 0x2, 0xab, 0xbc, 0x9f, 0x99, 0x23,
    0x19, 0x76, 0xa, 0x8c, 0xc4, 0x8b, 0xc7, 0xf3, 0x95, 0xd2, 0x90, 0xd2, 0xed, 0xd5, 0xcc, 0xf1, 0x31, 0x1d, 0xa7,
    0xa4, 0xba, 0xdd, 0x88, 0x2d, 0x6b, 0xa4, 0x74, 0xbb, 0x75, 0x73, 0x3c, 0x4c, 0x44, 0x42, 0xd, 0x74, 0x61, 0x39,
    0x79, 0x76, 0xa4, 0xa7, 0x3c, 0x4c, 0x80, 0xc4, 0xc8, 0x1a, 0x27, 0x1c, 0xe4, 0xde, 0x8, 0xb2, 0xba, 0x30, 0xa5,
    0x74, 0x36, 0x76, 0xa6, 0x53, 0x9f, 0x33, 0x56, 0x98, 0x88, 0x92, 0x2a, 0xd1, 0x90, 0x1,
];

const DVC_TEST_HEADER_SIZE: usize = 0x01;

lazy_static! {
    static ref DVC_FULL_DATA_FIRST_BUFFER: Vec<u8> = {
        let mut result = DVC_DATA_FIRST_PREFIX.to_vec();
        result.extend(DVC_DATA_FIRST_BUFFER);

        result
    };
    static ref FULL_DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_BUFFER: Vec<u8> = {
        let mut result = DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_PREFIX.to_vec();
        result.append(&mut DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_BUFFER.to_vec());

        result
    };
    static ref DVC_DATA_FIRST: DataFirstPdu = DataFirstPdu {
        channel_id_type: FieldType::U8,
        channel_id: DVC_TEST_CHANNEL_ID_U8,
        total_data_size_type: FieldType::U16,
        total_data_size: DVC_TEST_DATA_LENGTH,
        data_size: DVC_DATA_FIRST_BUFFER.len()
    };
    static ref DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH: ClientPdu = ClientPdu::DataFirst(DataFirstPdu {
        channel_id_type: FieldType::U8,
        channel_id: 0x7,
        total_data_size_type: FieldType::U16,
        total_data_size: 0x639,
        data_size: DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_BUFFER.len(),
    });
}

#[test]
fn from_buffer_parsing_for_dvc_data_first_pdu_with_invalid_message_size_fails() {
    let mut cur = ReadCursor::new(DVC_INVALID_DATA_MESSAGE_BUFFER.as_ref());
    match DataFirstPdu::decode(&mut cur, FieldType::U8, FieldType::U16, PDU_WITH_DATA_MAX_SIZE) {
        Err(e) if matches!(e.kind(), PduErrorKind::InvalidMessage { .. }) => (),
        res => panic!("Expected InvalidMessage error, got: {res:?}"),
    };
}

#[test]
fn from_buffer_parsing_for_dvc_data_first_pdu_with_invalid_total_message_size_fails() {
    let mut cur = ReadCursor::new(DVC_DATA_FIRST_WITH_INVALID_TOTAL_MESSAGE_SIZE_BUFFER.as_ref());
    match DataFirstPdu::decode(
        &mut cur,
        FieldType::U8,
        FieldType::U8,
        DVC_DATA_FIRST_WITH_INVALID_TOTAL_MESSAGE_SIZE_BUFFER_SIZE,
    ) {
        Err(e) if matches!(e.kind(), PduErrorKind::NotEnoughBytes { .. }) => (),
        res => panic!("Expected NotEnoughBytes error, got: {res:?}"),
    };
}

#[test]
fn from_buffer_correct_parses_dvc_data_first_pdu() {
    let mut cur = ReadCursor::new(&DVC_FULL_DATA_FIRST_BUFFER[1..]);
    assert_eq!(
        *DVC_DATA_FIRST,
        DataFirstPdu::decode(
            &mut cur,
            FieldType::U8,
            FieldType::U16,
            DVC_FULL_DATA_FIRST_BUFFER_SIZE - DVC_TEST_HEADER_SIZE
        )
        .unwrap(),
    );
}

#[test]
fn to_buffer_correct_serializes_dvc_data_first_pdu() {
    let data_first = &*DVC_DATA_FIRST;

    let buffer = encode_vec(data_first).unwrap();

    assert_eq!(DVC_DATA_FIRST_PREFIX.as_ref(), buffer.as_slice());
}

#[test]
fn buffer_length_is_correct_for_dvc_data_first_pdu() {
    let data_first = &*DVC_DATA_FIRST;
    let expected_buf_len = DVC_DATA_FIRST_PREFIX.len();

    let len = data_first.size();

    assert_eq!(expected_buf_len, len);
}

#[test]
fn from_buffer_correct_parses_dvc_server_pdu_with_data_first_where_total_length_equals_to_buffer_length() {
    let mut cur = ReadCursor::new(FULL_DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_BUFFER.as_slice());
    assert_eq!(
        *DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH,
        ClientPdu::decode(
            &mut cur,
            FULL_DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_BUFFER.len(),
        )
        .unwrap(),
    );
}

#[test]
fn to_buffer_correct_serializes_dvc_server_pdu_with_data_first_where_total_length_equals_to_buffer_length() {
    let data_first = &*DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH;

    let buffer = encode_vec(data_first).unwrap();

    assert_eq!(
        DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_PREFIX.as_ref(),
        buffer.as_slice()
    );
}

#[test]
fn buffer_length_is_correct_for_dvc_server_pdu_with_data_first_where_total_length_equals_to_buffer_length() {
    let data_first = &*DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH;
    let expected_buf_len = DATA_FIRST_WHERE_TOTAL_LENGTH_EQUALS_TO_BUFFER_LENGTH_PREFIX.len();

    let len = data_first.size();

    assert_eq!(expected_buf_len, len);
}
