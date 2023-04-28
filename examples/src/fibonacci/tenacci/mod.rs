use super::utils::compute_tenacci_term;
use crate::{Blake3_192, Blake3_256, Example, ExampleOptions, HashFunction, Sha3_256};
use core::marker::PhantomData;
use std::time::Instant;

use log::debug;
use winterfell::{
    crypto::{DefaultRandomCoin, ElementHasher},
    math::{fields::f128::BaseElement, FieldElement},
    ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};

mod air;
use air::TenAcciAir;

mod prover;
use prover::TenAcciProver;

// CONSTANTS
// ================================================================================================

const TRACE_WIDTH: usize = 10;

// TENACCI EXAMPLE
// ================================================================================================

pub fn get_example(
    options: &ExampleOptions,
    sequence_length: usize,
) -> Result<Box<dyn Example>, String> {
    let (options, hash_fn) = options.to_proof_options(28, 8);

    match hash_fn {
        HashFunction::Blake3_192 => Ok(Box::new(TenAcciExample::<Blake3_192>::new(
            sequence_length,
            options,
        ))),
        HashFunction::Blake3_256 => Ok(Box::new(TenAcciExample::<Blake3_256>::new(
            sequence_length,
            options,
        ))),
        HashFunction::Sha3_256 => Ok(Box::new(TenAcciExample::<Sha3_256>::new(
            sequence_length,
            options,
        ))),
        _ => Err("The specified hash function cannot be used with this example.".to_string()),
    }
}

pub struct TenAcciExample<H: ElementHasher> {
    options: ProofOptions,
    sequence_length: usize,
    result: BaseElement,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> TenAcciExample<H> {
    pub fn new(sequence_length: usize, options: ProofOptions) -> Self {
        assert_eq!(sequence_length % 10, 0, "sequence is not a multiple of ten");

        //Compute tenacci sequence
        let now = Instant::now();
        let result = compute_tenacci_term(sequence_length);
        debug!(
            "Computed Tenacci sequence up to {}th term in {} ms",
            sequence_length,
            now.elapsed().as_millis()
        );

        TenAcciExample {
            options,
            sequence_length,
            result,
            _hasher: PhantomData,
        }
    }
}


// EXAMPLE IMPLEMENTATION
// ================================================================================================

impl<H: ElementHasher> Example for TenAcciExample<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    fn prove(&self) -> StarkProof {
        debug!(
            "Generating proof for computing Fibonacci sequence (2 terms per step) up to {}th term\n\
            ---------------------",
            self.sequence_length
        );

        // create a prover
        let prover = TenAcciProver::<H>::new(self.options.clone());

        // generate execution trace
        let now = Instant::now();
        let trace = prover.build_trace(self.sequence_length);

        let trace_width = trace.width();
        let trace_length = trace.length();
        debug!(
            "Generated execution trace of {} registers and 2^{} steps in {} ms",
            trace_width,
            trace_length.ilog2(),
            now.elapsed().as_millis()
        );

        // generate the proof
        prover.prove(trace).unwrap()
    }
    
    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<TenAcciAir, H, DefaultRandomCoin<H>>(proof, self.result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<TenAcciAir, H, DefaultRandomCoin<H>>(proof, self.result + BaseElement::ONE)
    }
}


























