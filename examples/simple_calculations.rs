use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::witness::{PartialWitness, WitnessWrite, Witness};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2_field::goldilocks_field::GoldilocksField;

/// An example of using Plonky2 to prove a statement of the form
/// "I know a_1*w_1+a_2*w_2".
fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let a_1 = builder.add_virtual_target();
    let a_2 = builder.add_virtual_target();
    let w_1 = builder.add_virtual_target();
    let w_2 = builder.add_virtual_target();
    let aw_1 = builder.mul(a_1, w_1);
    let aw_2 = builder.mul(a_2, w_2);
    let out = builder.add(aw_1, aw_2);
    // let mut cur_target = initial;
    // for i in 2..101 {
    //     let i_target = builder.constant(F::from_canonical_u32(i));
    //     cur_target = builder.mul(cur_target, i_target);
    // }

    // Public inputs are the initial value (provided below) and the result (which is generated).
    builder.register_public_input(w_1);
    builder.register_public_input(w_2);
    builder.register_public_input(out);

    let mut pw = PartialWitness::new();
    pw.set_target(w_1, F::from(GoldilocksField(5)));
    pw.set_target(w_2, F::ONE);
    pw.set_target(a_1, F::ONE);
    pw.set_target(a_2, F::TWO);
    pw.set_target(out, F::from(GoldilocksField(7)));

    let data = builder.build::<C>();
    let proof = data.prove(pw.clone())?;

    println!(
        "w_1 {} * a_1 {} + w_2 {} * a_2 {} = out {}",
        proof.public_inputs[0], pw.get_target(a_1),
        proof.public_inputs[1], pw.get_target(a_2),
        proof.public_inputs[2]
    );

    data.verify(proof)
}
