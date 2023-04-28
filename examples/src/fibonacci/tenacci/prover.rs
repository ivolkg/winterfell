use super::{
    air::TenAcciAir, BaseElement, DefaultRandomCoin, ElementHasher, FieldElement, PhantomData,
    ProofOptions, Prover, Trace, TraceTable, TRACE_WIDTH,
};

// TENACCI PROVER
// ================================================================================================

pub struct TenAcciProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> TenAcciProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    /// Builds an execution trace for computin a Tenacci sequence of the specified lenght such
    /// that each row advances the sequence by 10 terms.
    pub fn build_trace(&self, sequence_length: usize) -> TraceTable<BaseElement> {
        /* assert!(
            sequence_lenght.is_power_of_two(),
            "sequence lenght must be a power of 2"
        ); */

        assert_eq!(
            sequence_length % 10,
            0,
            "sequence lenght must be a multiple of ten"
        );

        let mut trace = TraceTable::new(TRACE_WIDTH, sequence_length / 10);
        let two = BaseElement::ONE + BaseElement::ONE;
        trace.fill(
            |state| {
                state[0] = BaseElement::ONE;
                state[1] = BaseElement::ONE;
                state[2] = two;
                state[3] = two.exp(2);
                state[4] = two.exp(3);
                state[5] = two.exp(4);
                state[6] = two.exp(5);
                state[7] = two.exp(6);
                state[8] = two.exp(7);
                state[9] = two.exp(8);
            },
            |_, state| {
                let mut temp = state.to_owned();
                for _ in 0..temp.len() {
                    let mut next = temp.iter().fold(BaseElement::ONE, |acc, &val| acc + val);
                    temp.rotate_left(1);
                    core::mem::swap(&mut next, &mut temp.last_mut().unwrap());
                }
                for i in 0..state.len() {
                    state[i] = temp[i];
                }
            },
        );

        trace
    }
}

impl<H: ElementHasher> Prover for TenAcciProver<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    type BaseField = BaseElement;
    type Air = TenAcciAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = H;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        let last_step = trace.length() - 1;
        trace.get(9, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
