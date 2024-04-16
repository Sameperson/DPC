use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use bls12_381::{Bls12, Scalar};
use ff::Field;
use rand::rngs::OsRng;

#[derive(Clone)]
pub struct EqualityCircuit {
    a: Option<Scalar>,
    b: Option<Scalar>
}

impl Circuit<Scalar> for EqualityCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let a_var = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b_var = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?;

        // Enforce a == b
        cs.enforce(|| "a equals b", |lc| lc + a_var, |lc| lc + CS::one(), |lc| lc + b_var);

        Ok(())
    }
}


pub fn setup_and_test() {
    let rng = &mut OsRng;
    let circuit = EqualityCircuit { a: Some(Scalar::one()), b: Some(Scalar::one()) };

    let params = generate_random_parameters::<Bls12, _, _>(circuit.clone(), rng).unwrap();
    let pvk = prepare_verifying_key(&params.vk);

    let proof = create_random_proof(circuit, &params, rng).unwrap();

    let public_inputs = vec![Scalar::one(), Scalar::one()];  // The public inputs must match those in the circuit

    match verify_proof(&pvk, &proof, &public_inputs) {
        Ok(()) => println!("Proof verified successfully!"),
        Err(err) => panic!("Proof verification failed: {:?}", err),
    }
}




