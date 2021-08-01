//! Number Theoretic Trasform (NTT)
//!
//! NTT operations and operations performed in the NTT domain

use crate::polyvec::structures::{FiniteField, FiniteRing, RingModule};
use crate::{Poly3329, PolyMatrix3329, PolyVec3329, F3329};

/// 256-roots of unity
const ZETAS_256: [usize; 256] = [
    1, 17, 289, 1584, 296, 1703, 2319, 2804, 1062, 1409, 650, 1063, 1426, 939, 2647, 1722, 2642,
    1637, 1197, 375, 3046, 1847, 1438, 1143, 2786, 756, 2865, 2099, 2393, 733, 2474, 2110, 2580,
    583, 3253, 2037, 1339, 2789, 807, 403, 193, 3281, 2513, 2773, 535, 2437, 1481, 1874, 1897,
    2288, 2277, 2090, 2240, 1461, 1534, 2775, 569, 3015, 1320, 2466, 1974, 268, 1227, 885, 1729,
    2761, 331, 2298, 2447, 1651, 1435, 1092, 1919, 2662, 1977, 319, 2094, 2308, 2617, 1212, 630,
    723, 2304, 2549, 56, 952, 2868, 2150, 3260, 2156, 33, 561, 2879, 2337, 3110, 2935, 3289, 2649,
    1756, 3220, 1476, 1789, 452, 1026, 797, 233, 632, 757, 2882, 2388, 648, 1029, 848, 1100, 2055,
    1645, 1333, 2687, 2402, 886, 1746, 3050, 1915, 2594, 821, 641, 910, 2154, 3328, 3312, 3040,
    1745, 3033, 1626, 1010, 525, 2267, 1920, 2679, 2266, 1903, 2390, 682, 1607, 687, 1692, 2132,
    2954, 283, 1482, 1891, 2186, 543, 2573, 464, 1230, 936, 2596, 855, 1219, 749, 2746, 76, 1292,
    1990, 540, 2522, 2926, 3136, 48, 816, 556, 2794, 892, 1848, 1455, 1432, 1041, 1052, 1239, 1089,
    1868, 1795, 554, 2760, 314, 2009, 863, 1355, 3061, 2102, 2444, 1600, 568, 2998, 1031, 882,
    1678, 1894, 2237, 1410, 667, 1352, 3010, 1235, 1021, 712, 2117, 2699, 2606, 1025, 780, 3273,
    2377, 461, 1179, 69, 1173, 3296, 2768, 450, 992, 219, 394, 40, 680, 1573, 109, 1853, 1540,
    2877, 2303, 2532, 3096, 2697, 2572, 447, 941, 2681, 2300, 2481, 2229, 1274, 1684, 1996, 642,
    927, 2443, 1583, 279, 1414, 735, 2508, 2688, 2419, 1175,
];

/// 7-byte reversal (to impleme)
pub fn byte_rev(i: usize) -> usize {
    i
}

/// Basecase multiplication between polynomials (p 7)
pub fn bcm(a: &Poly3329, b: &Poly3329) -> Poly3329 {
    assert_eq!(a.degree(), b.degree());
    assert_eq!(a.dimension(), b.dimension());

    // BCM with the zero polynomial is the zero polynomial
    if a.is_zero() || b.is_zero() {
        return a.zero();
    }
    // Unwraps safely since the case None has been tested above
    let d = a.degree().unwrap();

    let mut p = Poly3329::init(a.dimension());

    for i in 0..=d / 2 {
        let zeta = F3329::from_int(ZETAS_256[2 * byte_rev(i) + 1]);

        let p01 = a[2 * i].mul(&b[2 * i]);
        let p02 = a[2 * i + 1].mul(&b[2 * i + 1]).mul(&zeta);
        p[2 * i] = p01.add(&p02);

        let p11 = a[2 * i].mul(&b[2 * i + 1]);
        let p12 = a[2 * i + 1].mul(&b[2 * i]);
        p[2 * i + 1] = p11.add(&p12);
    }
    p
}

/// Base case multiplivation for vectors
pub fn bcm_vec(a: &PolyVec3329, b: &PolyVec3329) -> Poly3329 {
    let l = a.dimension();
    assert_eq!(l, b.dimension());

    let mut p = bcm(&a.get(0), &b.get(0));
    for i in 1..l {
        p = p.add(&bcm(&a.get(i), &b.get(i)));
    }
    p
}

/// Matrix basecase multiplication, cf p. 7
pub fn bcm_matrix_vec(a: &PolyMatrix3329, b: &PolyVec3329) -> PolyVec3329 {
    let (x, y) = a.dimensions();
    assert_eq!(x, b.dimension());

    let mut v = PolyVec3329::init(y);

    for i in 0..y {
        v.set(i, bcm_vec(&a.row(i), &b))
    }

    v
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product(a_hat: &Poly3329, b_hat: &Poly3329) -> Poly3329 {
    rev_ntt(&bcm(a_hat, b_hat))
}

/// Computes a^T.b as NTT^-1(a_hat^T o b_hat)
pub fn ntt_product_vec(a_hat: &PolyVec3329, b_hat: &PolyVec3329) -> Poly3329 {
    rev_ntt(&bcm_vec(a_hat, b_hat))
}

/// Computes a.b as NTT^-1(a_hat o b_hat)
pub fn ntt_product_matvec(a_hat: &PolyMatrix3329, b_hat: &PolyVec3329) -> PolyVec3329 {
    rev_ntt_vec(&bcm_matrix_vec(a_hat, b_hat))
}

/// Number theoretic Transform on vectors
pub fn ntt_vec(p: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for p_i in p.coefficients.iter() {
        c.push(base_ntt(p_i));
    }
    PolyVec3329::from_vec(c)
}

/// Reverse NTT on vectors
pub fn rev_ntt_vec(p_hat: &PolyVec3329) -> PolyVec3329 {
    let mut c = vec![];
    for p_i in p_hat.coefficients.iter() {
        c.push(rev_ntt(p_i));
    }
    PolyVec3329::from_vec(c)
}

/// Number theoretic Transform
pub fn base_ntt(p: &Poly3329) -> Poly3329 {
    let mut a = Poly3329::init(p.dimension());

    // Zero polynomial's NTT is zero
    if p.is_zero() {
        return p.zero();
    }
    // Unwraps safely since the case None has been tested above
    let d = p.degree().unwrap();

    // We assume d is even since spec requires operating mod X^2-zeta
    for i in 0..=d / 2 {
        let mut p0 = p[0];
        let mut p1 = p[1];

        for j in 1..=d / 2 {
            let index = (2 * byte_rev(i) * j + j) % 256;
            let zeta = F3329::from_int(ZETAS_256[index]);
            let mut c0 = p[2 * j];
            let mut c1 = p[2 * j + 1];

            c0 = c0.mul(&zeta);
            c1 = c1.mul(&zeta);

            p0 = p0.add(&c0);
            p1 = p1.add(&c1);
        }
        a[2 * i] = p0;
        a[2 * i + 1] = p1;
    }

    a
}

/// Reverse NTT
pub fn rev_ntt(p_hat: &Poly3329) -> Poly3329 {
    let mut a = Poly3329::init(p_hat.dimension());

    // Zero polynomial's NTT is zero
    if p_hat.is_zero() {
        return p_hat.zero();
    }
    // Unwraps safely since the case None has been tested above
    let d = p_hat.degree().unwrap();

    let coeff = F3329::from_int((d / 2) + 1);

    for i in 0..=d / 2 {
        let mut p0 = p_hat[0];
        let mut p1 = p_hat[1];
        let z = F3329::from_int(ZETAS_256[((256 - i) % 256)]);

        for j in 1..=d / 2 {
            let index = (2 * byte_rev(i) * j) % 256;
            let zeta = F3329::from_int(ZETAS_256[(256 - index) % 256]);
            let mut c0 = p_hat[2 * j];
            let mut c1 = p_hat[2 * j + 1];

            c0 = c0.mul(&zeta);
            c1 = c1.mul(&zeta);

            p0 = p0.add(&c0);
            p1 = p1.add(&c1);
        }

        // Unwraps safely since coeff is d/2 + 1
        a[2 * i] = p0.mul(&z).div(&coeff).unwrap();
        a[2 * i + 1] = p1.mul(&z).div(&coeff).unwrap();
    }

    a
}

#[test]
fn rev_then_ntt() {
    let mut u_bold = Poly3329::from_vec(vec![Default::default(); 256], 256);
    for i in 0..256 {
        u_bold[i] = F3329::from_int(i);
    }
    let u = rev_ntt(&u_bold);

    assert_eq!(u_bold.coefficients, base_ntt(&u).coefficients)
}

#[test]
fn ntt_then_rev() {
    let mut u = Poly3329::from_vec(vec![Default::default(); 256], 256);
    for i in 0..256 {
        u[i] = F3329::from_int(i);
    }
    let u_bold = base_ntt(&u);

    assert_eq!(u.coefficients, rev_ntt(&u_bold).coefficients)
}
