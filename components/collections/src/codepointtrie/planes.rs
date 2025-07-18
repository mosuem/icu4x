// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Sample data for [`CodePointTrie`] that returns the code point's plane number.

use crate::codepointtrie::cptrie::*;
use zerovec::ZeroVec;

const INDEX_ARRAY_AS_BYTES: &[u8] = &[
    0x0, 0x0, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x88, 0x2, 0x90, 0x2,
    0x90, 0x2, 0x90, 0x2, 0xb0, 0x2, 0xb0, 0x2, 0xb0, 0x2, 0xb0, 0x2, 0xd0, 0x2, 0xd0, 0x2, 0xd0,
    0x2, 0xd0, 0x2, 0xf0, 0x2, 0xf0, 0x2, 0xf0, 0x2, 0xf0, 0x2, 0x10, 0x3, 0x10, 0x3, 0x10, 0x3,
    0x10, 0x3, 0x30, 0x3, 0x30, 0x3, 0x30, 0x3, 0x30, 0x3, 0x50, 0x3, 0x50, 0x3, 0x50, 0x3, 0x50,
    0x3, 0x70, 0x3, 0x70, 0x3, 0x70, 0x3, 0x70, 0x3, 0x90, 0x3, 0x90, 0x3, 0x90, 0x3, 0x90, 0x3,
    0xb0, 0x3, 0xb0, 0x3, 0xb0, 0x3, 0xb0, 0x3, 0xd0, 0x3, 0xd0, 0x3, 0xd0, 0x3, 0xd0, 0x3, 0xf0,
    0x3, 0xf0, 0x3, 0xf0, 0x3, 0xf0, 0x3, 0x10, 0x4, 0x10, 0x4, 0x10, 0x4, 0x10, 0x4, 0x30, 0x4,
    0x30, 0x4, 0x30, 0x4, 0x30, 0x4, 0x50, 0x4, 0x50, 0x4, 0x50, 0x4, 0x50, 0x4, 0x70, 0x4, 0x70,
    0x4, 0x70, 0x4, 0x70, 0x4, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30, 0x0, 0x40, 0x0, 0x50, 0x0,
    0x60, 0x0, 0x70, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20,
    0x0, 0x30, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0,
    0x30, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30,
    0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30, 0x0, 0x0, 0x0, 0x10, 0x0, 0x20, 0x0, 0x30, 0x0,
    0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80,
    0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0,
    0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80,
    0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x80, 0x0, 0x90, 0x0, 0x90, 0x0,
    0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90,
    0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0,
    0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90,
    0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0x90, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0,
    0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0,
    0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0,
    0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0, 0x0, 0xa0,
    0x0, 0xa0, 0x0, 0xa0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0,
    0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0,
    0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0,
    0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0, 0x0, 0xb0,
    0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0,
    0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0,
    0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0,
    0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xc0, 0x0, 0xd0, 0x0, 0xd0,
    0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0,
    0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0,
    0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0,
    0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xd0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0,
    0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0,
    0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0,
    0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0,
    0xe0, 0x0, 0xe0, 0x0, 0xe0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0,
    0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0,
    0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0,
    0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0, 0xf0, 0x0,
    0xf0, 0x0, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0,
    0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1,
    0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0,
    0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x0, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10,
    0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1,
    0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10,
    0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1, 0x10, 0x1,
    0x10, 0x1, 0x10, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20,
    0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1,
    0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20,
    0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1, 0x20, 0x1,
    0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30,
    0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1,
    0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30,
    0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x30, 0x1, 0x40, 0x1, 0x40, 0x1,
    0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40,
    0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1,
    0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40,
    0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x40, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1,
    0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50,
    0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1,
    0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50, 0x1, 0x50,
    0x1, 0x50, 0x1, 0x50, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1,
    0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60,
    0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1,
    0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60, 0x1, 0x60,
    0x1, 0x80, 0x0, 0x88, 0x0, 0x88, 0x0, 0x88, 0x0, 0x88, 0x0, 0x88, 0x0, 0x88, 0x0, 0x88, 0x0,
    0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2,
    0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0,
    0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0x2,
    0x0, 0x2, 0x0, 0x2, 0x0, 0x2, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8,
    0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0,
    0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8,
    0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0, 0xa8, 0x0,
    0xa8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8,
    0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0,
    0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8,
    0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xc8, 0x0, 0xe8, 0x0,
    0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8,
    0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0,
    0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8,
    0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0xe8, 0x0, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8,
    0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1,
    0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8,
    0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1, 0x8, 0x1,
    0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28,
    0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1,
    0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28,
    0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x28, 0x1, 0x48, 0x1, 0x48, 0x1,
    0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48,
    0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1,
    0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48,
    0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x48, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1,
    0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68,
    0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1,
    0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68, 0x1, 0x68,
    0x1, 0x68, 0x1, 0x68, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1,
    0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88,
    0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1,
    0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88, 0x1, 0x88,
    0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1,
    0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8,
    0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1,
    0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xa8, 0x1, 0xc8, 0x1, 0xc8,
    0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1,
    0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8,
    0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1,
    0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xc8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8,
    0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1,
    0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8,
    0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1,
    0xe8, 0x1, 0xe8, 0x1, 0xe8, 0x1, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2,
    0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8,
    0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2,
    0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x8, 0x2, 0x28, 0x2, 0x28, 0x2,
    0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28,
    0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2,
    0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28,
    0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x28, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2,
    0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48,
    0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2,
    0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48, 0x2, 0x48,
    0x2, 0x48, 0x2, 0x48, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2,
    0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68,
    0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2,
    0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68, 0x2, 0x68,
    0x2,
];

/// Return a [`CodePointTrie`] that returns the Unicode plane number, an
/// integer from 0-16 inclusive, for each code point.
///
/// This `CodePointTrie`
/// does not actually represent any Unicode property, but it is provided in
/// case it is useful to users of `CodePointTrie` for testing or other
/// purposes. See <https://www.unicode.org/glossary/#plane>.
pub fn get_planes_trie() -> CodePointTrie<'static, u8> {
    let index_array_as_bytes: &[u8] = INDEX_ARRAY_AS_BYTES;
    let data_8_array: &[u8] = &[
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 6,
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
        9, 9, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xa, 0xb,
        0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xb, 0xc, 0xc, 0xc,
        0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xc, 0xd, 0xd, 0xd, 0xd, 0xd,
        0xd, 0xd, 0xd, 0xd, 0xd, 0xd, 0xd, 0xd, 0xd, 0xd, 0xd, 0xe, 0xe, 0xe, 0xe, 0xe, 0xe, 0xe,
        0xe, 0xe, 0xe, 0xe, 0xe, 0xe, 0xe, 0xe, 0xe, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
        0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x10, 0x10, 0x10, 0,
    ];
    #[expect(clippy::unwrap_used)] // valid bytes
    let index: ZeroVec<u16> = ZeroVec::parse_bytes(index_array_as_bytes).unwrap();
    #[expect(clippy::unwrap_used)] // valid bytes
    let data: ZeroVec<u8> = ZeroVec::parse_bytes(data_8_array).unwrap();
    let high_start = 0x100000;
    let shifted12_high_start = 0x100;
    let index3_null_offset = 0x2;
    let data_null_offset = 0x0;
    let null_value = 0x0;
    let trie_type = TrieType::Small;

    let trie_header = CodePointTrieHeader {
        high_start,
        shifted12_high_start,
        index3_null_offset,
        data_null_offset,
        null_value,
        trie_type,
    };

    #[expect(clippy::unwrap_used)] // valid data
    CodePointTrie::try_new(trie_header, index, data).unwrap()
}

#[cfg(test)]
mod tests {
    use zerovec::ZeroVec;

    const INDEX_ARRAY: &[u16] = &[
        0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0x288, 0x290, 0x290, 0x290, 0x2b0, 0x2b0, 0x2b0, 0x2b0, 0x2d0, 0x2d0, 0x2d0,
        0x2d0, 0x2f0, 0x2f0, 0x2f0, 0x2f0, 0x310, 0x310, 0x310, 0x310, 0x330, 0x330, 0x330, 0x330,
        0x350, 0x350, 0x350, 0x350, 0x370, 0x370, 0x370, 0x370, 0x390, 0x390, 0x390, 0x390, 0x3b0,
        0x3b0, 0x3b0, 0x3b0, 0x3d0, 0x3d0, 0x3d0, 0x3d0, 0x3f0, 0x3f0, 0x3f0, 0x3f0, 0x410, 0x410,
        0x410, 0x410, 0x430, 0x430, 0x430, 0x430, 0x450, 0x450, 0x450, 0x450, 0x470, 0x470, 0x470,
        0x470, 0, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0, 0x10, 0x20, 0x30, 0, 0x10, 0x20,
        0x30, 0, 0x10, 0x20, 0x30, 0, 0x10, 0x20, 0x30, 0, 0x10, 0x20, 0x30, 0, 0x10, 0x20, 0x30,
        0, 0x10, 0x20, 0x30, 0, 0x10, 0x20, 0x30, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
        0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0xa0, 0xa0, 0xa0, 0xa0,
        0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0,
        0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xb0, 0xb0,
        0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0,
        0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0, 0xb0,
        0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
        0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0, 0xc0,
        0xc0, 0xc0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0,
        0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0, 0xd0,
        0xd0, 0xd0, 0xd0, 0xd0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
        0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0,
        0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xe0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
        0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0,
        0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100,
        0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100,
        0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100, 0x100,
        0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110,
        0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x110,
        0x110, 0x110, 0x110, 0x110, 0x110, 0x110, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120,
        0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120,
        0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x120, 0x130,
        0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130,
        0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130, 0x130,
        0x130, 0x130, 0x130, 0x130, 0x130, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140,
        0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140,
        0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x140, 0x150, 0x150,
        0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150,
        0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150, 0x150,
        0x150, 0x150, 0x150, 0x150, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160,
        0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160,
        0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x160, 0x80, 0x88, 0x88,
        0x88, 0x88, 0x88, 0x88, 0x88, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8,
        0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8,
        0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xa8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8,
        0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8,
        0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xc8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8,
        0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8,
        0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0xe8, 0x108, 0x108,
        0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108,
        0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108, 0x108,
        0x108, 0x108, 0x108, 0x108, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128,
        0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128,
        0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x128, 0x148, 0x148, 0x148,
        0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148,
        0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148, 0x148,
        0x148, 0x148, 0x148, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168,
        0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168,
        0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x168, 0x188, 0x188, 0x188, 0x188,
        0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188,
        0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188, 0x188,
        0x188, 0x188, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8,
        0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8,
        0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1a8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8,
        0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8,
        0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8, 0x1c8,
        0x1c8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8,
        0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8,
        0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x1e8, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208,
        0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208,
        0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208, 0x208,
        0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228,
        0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x228,
        0x228, 0x228, 0x228, 0x228, 0x228, 0x228, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248,
        0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248,
        0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x248, 0x268,
        0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268,
        0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268, 0x268,
        0x268, 0x268, 0x268, 0x268, 0x268,
    ];

    #[test]
    fn test_index_byte_array_literal() {
        let index_array_as_bytes: &[u8] = super::INDEX_ARRAY_AS_BYTES;
        let index_zv_bytes: ZeroVec<u16> =
            ZeroVec::parse_bytes(index_array_as_bytes).expect("infallible");
        let index_zv_aligned: ZeroVec<u16> = ZeroVec::from_slice_or_alloc(INDEX_ARRAY);
        assert_eq!(index_zv_bytes, index_zv_aligned);
    }
}
