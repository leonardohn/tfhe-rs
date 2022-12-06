//! Module containing functions related to LWE ciphertext linear algebra, like addition,
//! multiplication, etc.

use crate::core_crypto::algorithms::slice_algorithms::*;
use crate::core_crypto::commons::numeric::UnsignedInteger;
use crate::core_crypto::commons::traits::*;
use crate::core_crypto::entities::*;

pub fn lwe_ciphertext_in_place_addition<Scalar, LhsCont, RhsCont>(
    lhs: &mut LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    LhsCont: ContainerMut<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    update_slice_with_wrapping_add(lhs.as_mut(), rhs.as_ref());
}

pub fn lwe_ciphertext_addition<Scalar, OutputCont, LhsCont, RhsCont>(
    output: &mut LweCiphertext<OutputCont>,
    lhs: &LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    OutputCont: ContainerMut<Element = Scalar>,
    LhsCont: Container<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    slice_wrapping_add(output.as_mut(), lhs.as_ref(), rhs.as_ref());
}

pub fn lwe_ciphertext_in_place_plaintext_addition<Scalar, InCont>(
    lhs: &mut LweCiphertext<InCont>,
    rhs: Plaintext<Scalar>,
) where
    Scalar: UnsignedInteger,
    InCont: ContainerMut<Element = Scalar>,
{
    let body = lhs.get_mut_body();

    *body.0 = (*body.0).wrapping_add(rhs.0);
}

pub fn lwe_ciphertext_in_place_opposite<Scalar, InCont>(ct: &mut LweCiphertext<InCont>)
where
    Scalar: UnsignedInteger,
    InCont: ContainerMut<Element = Scalar>,
{
    update_slice_with_wrapping_opposite(ct.as_mut());
}

pub fn lwe_ciphertext_in_place_cleartext_multiplication<Scalar, InCont>(
    lhs: &mut LweCiphertext<InCont>,
    rhs: Cleartext<Scalar>,
) where
    Scalar: UnsignedInteger,
    InCont: ContainerMut<Element = Scalar>,
{
    update_slice_with_wrapping_scalar_mul(lhs.as_mut(), rhs.0);
}

pub fn lwe_ciphertext_in_place_subtraction<Scalar, LhsCont, RhsCont>(
    lhs: &mut LweCiphertext<LhsCont>,
    rhs: &LweCiphertext<RhsCont>,
) where
    Scalar: UnsignedInteger,
    LhsCont: ContainerMut<Element = Scalar>,
    RhsCont: Container<Element = Scalar>,
{
    update_slice_with_wrapping_sub(lhs.as_mut(), rhs.as_ref());
}

pub fn lwe_ciphertext_cleartext_multiplication<Scalar, InputCont, OutputCont>(
    output: &mut LweCiphertext<OutputCont>,
    lhs: &LweCiphertext<InputCont>,
    rhs: Cleartext<Scalar>,
) where
    Scalar: UnsignedInteger,
    InputCont: Container<Element = Scalar>,
    OutputCont: ContainerMut<Element = Scalar>,
{
    output.as_mut().copy_from_slice(lhs.as_ref());
    lwe_ciphertext_in_place_cleartext_multiplication(output, rhs);
}