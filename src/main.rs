use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::twist::BLS12381TwistCurve, point::ShortWeierstrassProjectivePoint,
        },
        traits::IsEllipticCurve,
    },
    unsigned_integer::element::{UnsignedInteger, U256},
};

pub struct SparklingWaterPK {}

pub trait SparklingWaterPKTrait {
    fn generate_public_key(secret_key: &str)
        -> ShortWeierstrassProjectivePoint<BLS12381TwistCurve>;
}

impl SparklingWaterPKTrait for SparklingWaterPK {
    fn generate_public_key(
        secret_key: &str,
    ) -> ShortWeierstrassProjectivePoint<BLS12381TwistCurve> {
        let secret_key: UnsignedInteger<4> = U256::from_hex(secret_key).unwrap();
        let public_key: ShortWeierstrassProjectivePoint<BLS12381TwistCurve> =
            BLS12381TwistCurve::generator().operate_with_self(secret_key);

        public_key.to_affine()
    }
}

fn main() {
    let public_key = SparklingWaterPK::generate_public_key("0x6C616D6264617370");
    println!("public_key={:?}", public_key);
}
