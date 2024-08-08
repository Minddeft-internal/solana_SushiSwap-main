
const U64_MAX: u128 = 18446744073709551615;

pub struct U256 {
    v0: u64,
    v1: u64,
    v2: u64,
    v3: u64,
}

pub fn mul_u128(a: u128, b: u128)-> U256 {
    let a1 = a >> 64;
    let a0 = a & U64_MAX;
    let b1 =b >> 64;
    let b0 = b & U64_MAX;
    // Step 1, v0 v1 created
    let p = a0 * b0;
    let mut v1 = p >> 64;
    let v0 = p & U64_MAX;
    // Step 2: v2 created, v1 modified
    let p = a1 * b0;
    v1 = v1 + (p & U64_MAX) ;
    let mut v2 = p >> 64;
    if v1 > U64_MAX {
        v2 = v2 + (v1 >> 64);
        v1 = v1 & U64_MAX;
    };
    // Step 3: v3 created, v2 v1 modified
    let p = a0 * b1;
    v1 = v1 + (p & U64_MAX);
    if v1 > U64_MAX {
        v2 = v2 + (v1 >> 64);
        v1 = v1 & U64_MAX;
    };
    v2 = v2 + (p >> 64);
    let mut v3 = v2 >> 64;
    if v2 > U64_MAX {
        v2 = v2 & U64_MAX;
    };
    // Step 4: v3 v2 modified
    let p = a1 * b1;
    v2 = v2 + (p & U64_MAX);
    if v2 > U64_MAX {
        v3 = v3 + (v2 >> 64);
        v2 = v2 & U64_MAX;
    };
    v3 = v3 + (p >> 64);
    // Result
    U256 { v0: (v0 as u64), v1: (v1 as u64), v2: (v2 as u64), v3: (v3 as u64) }
}

/// Greater or equal to
pub fn ge(a: &U256, b: &U256)-> bool {
    if a.v3 != b.v3 { return a.v3 >= b.v3 };
    if a.v2 != b.v2 { return a.v2 >= b.v2 };
    if a.v1 != b.v1 { return a.v1 >= b.v1 };
    a.v0 >= b.v0
}