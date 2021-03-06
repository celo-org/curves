use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::ops::{AddAssign, MulAssign, SubAssign};

use ark_ec::{
    mnt6::{G1Prepared, G2Prepared},
    PairingEngine, ProjectiveCurve,
};
use ark_ff::{
    biginteger::BigInteger768 as FqRepr, BigInteger, Field, PrimeField, SquareRootField,
    UniformRand,
};
use ark_mnt_753::mnt6_753::{
    fq::Fq, fq3::Fq3, fr::Fr, Fq6, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2,
    Parameters, MNT6_753,
};

ec_bench!();
f_bench!(1, Fq3, Fq3, fq3);
f_bench!(2, Fq6, Fq6, fq6);
f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
pairing_bench!(MNT6_753, Fq6, prepared_v);
