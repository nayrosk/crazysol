use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

pub fn compute_rate_of_centrifugation(supply: u128, deposit: u128, p0: u32, m: u8) -> u128 {
    let s = U256::from(supply);
    let p0 = U256::from(p0);
    let m = U256::from(m);
    let deposit = U256::from(deposit);

    let a = p0 + m * s;
    let sqrt_term = (a * a) + (U256::from(2) * m * deposit);
    let sqrt_val = sqrt_u256(sqrt_term);
    let delta_s = (sqrt_val - a) / m;

    delta_s.as_u128()
}


fn sqrt_u256(value: U256) -> U256 {
    if value.is_zero() {
        return U256::zero();
    }

    let mut z = value;
    let mut x = (value >> 1) + U256::one();
    while x < z {
        z = x;
        x = (value / x + x) >> 1;
    }
    z
}
