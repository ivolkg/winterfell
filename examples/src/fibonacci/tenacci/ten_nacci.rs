use anyhow::Result;
use plonky2::iop::witness::PartialWitness;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::field::types::Field;
use plonky2::plonk::verifier::verify;

fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();

    let pw = PartialWitness::new();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // Compute the 100th ten_nacci number
    let n = 100;
    let mut nums: [u64; 10] = [1, 1, 2, 4, 8, 16, 32, 64, 128, 256];
    for i in 10..n {
        let new_num = nums[i - 10..i].iter().sum();
        nums[i] = new_num;
    }
    let ten_nacci_100 = nums[n - 1];
    let ten_nacci_100_target = builder.constant(F::from_u64(ten_nacci_100 as u64));

    // Set up the circuit to verify the 24th ten_nacci number
    let result_target = builder.witness_unscaled(ten_nacci_24_target, &mut pw);
    let expected_target = builder.witness_unscaled(builder.constant(F::from_u64(ten_nacci(24) as u64)), &mut pw);
    builder.constrain_equal(result_target, expected_target);

    let data = builder.build::<C>();

    let proof = data.prove(pw)?;

    verify(proof, &data.verifier_only, &data.common)
}

// Original `ten_nacci` function
fn ten_nacci(n: u64) -> u64 {
    let nums: [u64; 10] = [1, 1, 2, 4, 8, 16, 32, 64, 128, 256];
    if n < 11 {
        return nums[n as usize - 1];
    } else {
        let mut nums: Vec<u64> = nums.to_vec();
        for i in 10..n {
            let new_num = nums[i as usize - 10..i as usize].iter().sum();
            nums.push(new_num);
        }
        return nums[n as usize - 1];
    }
}