#![allow(non_snake_case)]

use ark_ff::{Field, PrimeField, BigInteger};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, LinearCombination};
use ark_std::marker::PhantomData;

// =========================================================================
// PHASE 4.3: HYBRID MARKET-MAKER PRICING
// =========================================================================

#[allow(dead_code)]
#[derive(Clone)]
pub struct RideCircuit<F: Field> {
    pub active_drivers: Option<F>,
    pub active_riders: Option<F>,
    pub distance_miles: Option<F>,
    pub base_rate_per_mile: Option<F>,
    pub final_negotiated_fare: Option<F>,
    pub _engine: PhantomData<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for RideCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let dist_var = cs.new_witness_variable(|| self.distance_miles.ok_or(SynthesisError::AssignmentMissing))?;
        let base_rate_var = cs.new_input_variable(|| self.base_rate_per_mile.ok_or(SynthesisError::AssignmentMissing))?;
        let min_cost_val = self.distance_miles.and_then(|d| self.base_rate_per_mile.map(|r| d * r));
        let min_cost_var = cs.new_witness_variable(|| min_cost_val.ok_or(SynthesisError::AssignmentMissing))?;
        
        cs.enforce_constraint(
            LinearCombination::from(dist_var),
            LinearCombination::from(base_rate_var),
            LinearCombination::from(min_cost_var),
        )?;
        Ok(())
    }
}

// =========================================================================
// PHASE 1.6: ZK-DISTANCE BOUNDING (TIME-OF-FLIGHT)
// =========================================================================

#[derive(Clone)]
pub struct DistanceBoundingCircuit<F: PrimeField> {
    pub delta_t_nanos: Option<F>,        
    pub t_compute_nanos: Option<F>,      
    pub speed_of_light_mm_per_ns: Option<F>, 
    pub max_allowed_distance_mm: Option<F>,  
    pub _engine: PhantomData<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for DistanceBoundingCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let delta_t_var = cs.new_witness_variable(|| self.delta_t_nanos.ok_or(SynthesisError::AssignmentMissing))?;
        let t_compute_var = cs.new_witness_variable(|| self.t_compute_nanos.ok_or(SynthesisError::AssignmentMissing))?;
        let c_var = cs.new_input_variable(|| self.speed_of_light_mm_per_ns.ok_or(SynthesisError::AssignmentMissing))?;
        let max_dist_var = cs.new_input_variable(|| self.max_allowed_distance_mm.ok_or(SynthesisError::AssignmentMissing))?;

        let t_flight_val = self.delta_t_nanos.and_then(|dt| self.t_compute_nanos.map(|tc| dt - tc));
        let t_flight_var = cs.new_witness_variable(|| t_flight_val.ok_or(SynthesisError::AssignmentMissing))?;
        
        // Explicitly create a linear combination with a constant '1'
        let one: F = ark_ff::One::one();
        
        cs.enforce_constraint(
            LinearCombination::from(t_flight_var) + t_compute_var,
            LinearCombination::from((one, ark_relations::r1cs::Variable::One)),
            LinearCombination::from(delta_t_var),
        )?;

        let round_trip_dist_val = t_flight_val.and_then(|tf| self.speed_of_light_mm_per_ns.map(|c| tf * c));
        let round_trip_dist_var = cs.new_witness_variable(|| round_trip_dist_val.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(
            LinearCombination::from(t_flight_var),
            LinearCombination::from(c_var),
            LinearCombination::from(round_trip_dist_var),
        )?;

        let two_val = self.max_allowed_distance_mm.map(|_| F::from(2u32));
        let two_var = cs.new_input_variable(|| two_val.ok_or(SynthesisError::AssignmentMissing))?;
        let max_round_trip_val = self.max_allowed_distance_mm.and_then(|md| two_val.map(|two| md * two));
        let max_round_trip_var = cs.new_witness_variable(|| max_round_trip_val.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(
            LinearCombination::from(max_dist_var),
            LinearCombination::from(two_var),
            LinearCombination::from(max_round_trip_var),
        )?;

        // =========================================================================
        // PINNACLE OPTIMIZATION: DIFFERENCE RANGE PROOF (DRP)
        // =========================================================================
        
        // 1. Compute algebraic difference variable: diff = max_round_trip - round_trip_dist
        let diff_val = max_round_trip_val.and_then(|max_rt| {
            round_trip_dist_val.map(|rt_dist| max_rt - rt_dist)
        });
        let diff_var = cs.new_witness_variable(|| diff_val.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(
            LinearCombination::from(max_round_trip_var) - round_trip_dist_var,
            LinearCombination::from((one, ark_relations::r1cs::Variable::One)),
            LinearCombination::from(diff_var),
        )?;

        // 2. Proving diff is in [0, 2^32 - 1] via 32-bit decomposition
        let diff_bits = diff_val.map(|val| {
            let bigint = val.into_bigint();
            let mut bits = Vec::with_capacity(32);
            for i in 0..32 {
                bits.push(bigint.get_bit(i));
            }
            bits
        });

        let mut bit_vars = Vec::with_capacity(32);
        for i in 0..32 {
            let bit_val = diff_bits.as_ref().map(|bits| F::from(bits[i]));
            let bit_var = cs.new_witness_variable(|| bit_val.ok_or(SynthesisError::AssignmentMissing))?;

            // Enforce boolean constraint on each bit: bit * (1 - bit) = 0
            cs.enforce_constraint(
                LinearCombination::from(bit_var),
                LinearCombination::from((one, ark_relations::r1cs::Variable::One)) - bit_var,
                LinearCombination::zero(),
            )?;
            bit_vars.push(bit_var);
        }

        // 3. Enforce reconstruction constraint: sum(bit_i * 2^i) = diff
        let mut lc = LinearCombination::zero();
        let mut coeff = F::one();
        for &bit_var in &bit_vars {
            lc += (coeff, bit_var);
            coeff.double_in_place();
        }

        cs.enforce_constraint(
            lc,
            LinearCombination::from((one, ark_relations::r1cs::Variable::One)),
            LinearCombination::from(diff_var),
        )?;

        Ok(())
    }
}