use winterfell::{
    math::{fields::f128::BaseElement, FieldElement, ToElements},
    Air, AirContext, Assertion, ByteWriter, EvaluationFrame, FieldExtension, ProofOptions,
    StarkProof, TraceInfo, TraceTable, TransitionConstraintDegree, crypto::hashers::Blake3_256,
};

fn main() {
    let num_steps = 1024;
    let first_step = BaseElement::new(3);
    let last_step = do_work(first_step, num_steps);
    println!("Step {} is: {}", num_steps - 1, last_step);
    let trace = build_do_work_trace(first_step, num_steps);
}

fn do_work(start: BaseElement, n: usize) -> BaseElement {
    let mut result = start;
    for _ in 1..n {
        result = result.exp(3) + BaseElement::new(42);
    }
    result
}

pub fn build_do_work_trace(start: BaseElement, n: usize) -> TraceTable<BaseElement> {
    /* Instantiate the trace with a given width and lenght; this will allocate all
    required memory for the trace */
    let trace_width = 1;
    let mut trace = TraceTable::new(trace_width, n);

    /* Fill the trace with date; the first closure initializes the first state of the
    computations; the second closure computes the next state of the computations based
    on its current state */
    trace.fill(
        |state| {
            state[0] = start;
        },
        |_, state| {
            state[0] = state[0].exp(3u32.into()) + BaseElement::new(42);
        },
    );
    trace
}

// Public inputs for our computation will conssit of the starting value and the end result
pub struct PublicInputs {
    start: BaseElement,
    result: BaseElement,
}

// We need to describe how public inputs can be converted to field elements
impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![self.start, self.result]
    }
}

// For a specific instance of our computation, we'll keep track of the public inputs and
// the computation's context which we'll build in the constructor. The context is sued
// internally by the Winterfell prover/verifier when interpreting this AIR.
pub struct WorkAir {
    context: AirContext<BaseElement>,
    start: BaseElement,
    result: BaseElement,
}

impl Air for WorkAir {
    // First, we'll specify which finite field to sue for our computation, and also how
    // the public inputs must look like.
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;

    // Here, we'll construct a new instances of our computation which is defined by 3 parameters:
    // starting value, number of steps, and the end result. Another way to think about it is
    // that an instance of our computation is specific invocation of the do_wokr() function.
    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        // our execution trace should have only onw column
        assert_eq!(1, trace_info.width());

        // Our computation requires a single transition constraint. The constraint itself
        // is defined in the evalaute_transition() method bellow, but here we need to specify
        // the expected degree of the constraint. If the expected degree and the actual degree of
        // the cosntraint don't match, an error will be thrown in the debug mode, but in release
        // mode, an invalid proof will be generated which will not be accepted by any verifier.
        let degrees = vec![TransitionConstraintDegree::new(3)];

        // We also need to specify the exact number of assertions we will place against the
        // execution trace. This number be the same as the number of items in a vector
        // returned from the get_assertions() method below.
        let num_assertions = 2;

        WorkAir {
            context: AirContext::new(trace_info, degrees, num_assertions, options),
            start: pub_inputs.start,
            result: pub_inputs.result,
        }
    }

    // In this method we'll define our transtion constraints; a computation is considered to
    // be valid, if for all valid state transitions, transistion constraint evaluate to all
    // zeros, and for any invalid transition, at least one constraint evaluated to a non-zero
    // value. The `frame` parameter will contain current and next states of the computation.
    fn evaluate_transition<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        // First, we'll read the current state, and use it to compute the expected next state
        let current_state = frame.current()[0];
        let next_state = current_state.exp(3u32.into()) + E::from(42u32);

        // Then, we'll substract the expected next state from the actual next state; this will
        // evaluate to zero if and only if the expected and actual states are the same.
        result[0] = frame.next()[0] - next_state;
    }

    // Here, we'll define a set of assertions about the execution trace which must be satisfied
    // for the computation to be valid. Essentially, this ties computation's execution trace
    // to the public inputs
    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        // for our computation to be valid, valuein column 0 at step 0 must be equal to the
        // starting value, and at the last step it must be equal to the result.
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, self.start),
            Assertion::single(0, last_step, self.result),
        ]
    }

    // This is just boilerplate which is used by Winterfell prover-verifier to retrieve
    // the context of the computation.
    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
}

/* pub fn prove_work() -> (BaseElement, StarkProof){
    //We'll just hard-code the parameters here for this example.
    let start = BaseElement::new(3);
    let n = 1_048_576;

    // Build the execution trace and get the results from the last step.
    let trace = build_do_work_trace(start, n);
    let result = trace.get(0, n - 1);

    // Define proof options; these will be enough for ~96-bit security level.
    let options = ProofOptions::new(32, 8, 0, FieldExtension::None, 8, 128);

    // Instantiate the prover and generate the proof.
    let proof = prover.prove(trace).unwrap();

    (result, proof)
}

pub fn verify_work(start: BaseElement, result: BaseElement, proof: StarkProof) {
    // The number of steps and options are encoded in the proof itself, so we
    // don't need to pass them explicitly to the verifier.
    let pub_inputs = PublicInputs {start, result};
    match winterfell::verify::<WorkAir, Blake3_256<BaseField>>(proof, pub_inputs) {
        Ok(_) => println!("yay! all good!"),
        Err(_) => panic!("something went terribly wrong!"),
        
    }
} */
