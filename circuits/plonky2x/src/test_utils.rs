pub use std::str::FromStr;

pub use near_primitives::hash::CryptoHash;
pub use plonky2x::{
    backend::circuit::{PublicInput, PublicOutput},
    prelude::*,
};
pub use test_utils::*;

// Testnet Repr
pub const NETWORK: usize = 1;

pub type B<const D: usize = 2> = CircuitBuilder<DefaultParameters, D>;
pub type PI<const D: usize = 2> = PublicInput<DefaultParameters, D>;
pub type PO<const D: usize = 2> = PublicOutput<DefaultParameters, D>;

pub fn builder_suite<F, WriteInputs, Assertions>(
    define: F,
    writer: WriteInputs,
    assertions: Assertions,
) where
    F: FnOnce(&mut B),
    WriteInputs: FnOnce(&mut PI),
    Assertions: FnOnce(PO),
{
    pretty_env_logger::try_init().unwrap_or_default();

    let mut builder = B::new();
    define(&mut builder);

    let circuit = builder.build();

    let mut inputs = circuit.input();
    writer(&mut inputs);

    if let PublicInput::Bytes(bytes) = &mut inputs {
        std::fs::write("input.bin", hex!(bytes)).unwrap();
    }

    let (proof, output) = circuit.prove(&inputs);

    assertions(output.clone());

    circuit.verify(&proof, &inputs, &output);
}

pub fn mock_builder_suite<F, WriteInputs, Assertions>(
    define: F,
    writer: WriteInputs,
    assertions: Assertions,
) where
    F: FnOnce(&mut B),
    WriteInputs: FnOnce(&mut PI),
    Assertions: FnOnce(PO),
{
    pretty_env_logger::try_init().unwrap_or_default();

    let mut builder = B::new();
    define(&mut builder);

    let circuit = builder.mock_build();

    let mut inputs = circuit.input();
    writer(&mut inputs);

    let (witness, output) = circuit.mock_prove(&inputs);
    println!("Mock proof {:#?}", witness.full_witness());

    assertions(output.clone());
}
