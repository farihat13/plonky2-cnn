use anyhow::Result;
use env_logger::Target;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2_field::goldilocks_field::GoldilocksField;

/// An example of using Plonky2 to prove a statement of the form
/// "I know a_1*w_1+a_2*w_2+ ... + a_n*w_n".
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    const N: usize = 3;
    
    let mut weights = vec![];
    for _ in 0..N {
        weights.push(builder.add_virtual_target());
    }
    
    let mut values = vec![];
    for _ in 0..N {
        values.push(builder.add_virtual_target());
    }

    let mut mults = vec![];
    for i in 0..N {
        let mult = builder.mul(values[i], weights[i]);
        mults.push(mult);
    }

    let mut sum = mults[0];
    for i in 1..N {
        sum = builder.add(sum, mults[i]);
    }

    // Public inputs: sum.
    // for weight in &weights {
    //     builder.register_public_input(*weight);
    // }
    builder.register_public_input(sum);


    // witness
    let mut pw = PartialWitness::new();
    for (i, weight) in weights.iter().enumerate() {
        pw.set_target(*weight, F::from(GoldilocksField(i as u64)));
    }
    for (i, value) in values.iter().enumerate() {
        pw.set_target(*value, F::from(GoldilocksField( (i+1) as u64)));
    }


    pw.set_target(sum, F::from(GoldilocksField(8)));

    let data = builder.build::<C>();
    let proof = data.prove(pw.clone())?;

    println!("dot product {}", proof.public_inputs[0]);

    data.verify(proof)
}
