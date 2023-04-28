// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::borrow::BorrowMut;

use winterfell::math::{fields::f128::BaseElement, FieldElement};

pub fn compute_fib_term<E: FieldElement>(n: usize) -> E {
    let mut t0 = E::ONE;
    let mut t1 = E::ONE;

    for _ in 0..(n - 1) {
        t1 = t0 + t1;
        core::mem::swap(&mut t0, &mut t1);
    }

    t1
}

pub fn compute_tenacci_term<E: FieldElement>(n: usize) -> E {
    let mut ta = [E::ZERO; 10];
    ta[0] = E::ONE;
    ta[1] = E::ONE;
    for i in 2..10 {
        let mut sum = E::ZERO;
        for &val in ta.iter().take(i) {
           sum = sum + val; 
        }
        ta[i] = sum;
    }

    for _ in 0..(n-9) {
        let mut t10 = ta.iter().fold(E::ZERO, |acc, &val| acc + val);
        ta.rotate_left(1);
        core::mem::swap(&mut t10, &mut ta.last_mut().unwrap())
    }

    ta.last().unwrap_or(&E::ZERO).to_owned()
}

pub fn compute_mulfib_term(n: usize) -> BaseElement {
    let mut t0 = BaseElement::ONE;
    let mut t1 = BaseElement::new(2);

    for _ in 0..(n - 1) {
        t1 = t0 * t1;
        core::mem::swap(&mut t0, &mut t1);
    }

    t1
}

#[cfg(test)]
pub fn build_proof_options(use_extension_field: bool) -> winterfell::ProofOptions {
    use winterfell::{FieldExtension, ProofOptions};

    let extension = if use_extension_field {
        FieldExtension::Quadratic
    } else {
        FieldExtension::None
    };
    ProofOptions::new(28, 8, 0, extension, 4, 7)
}
