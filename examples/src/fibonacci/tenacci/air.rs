use crate::utils::are_equal;

use super::{BaseElement, TRACE_WIDTH};
use winterfell::{
    math::FieldElement, Air, AirContext, Assertion, EvaluationFrame, ProofOptions, TraceInfo,
    TransitionConstraintDegree,
};

// TENACCI AIR
// ================================================================================================

pub struct TenAcciAir {
    context: AirContext<BaseElement>,
    result: BaseElement,
}

impl Air for TenAcciAir {
    type BaseField = BaseElement;
    type PublicInputs = BaseElement;

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(trace_info: TraceInfo, pub_inputs: Self::BaseField, options: ProofOptions) -> Self {
        let degrees = vec![
            TransitionConstraintDegree::new(1),
            TransitionConstraintDegree::new(1),
        ];
        assert_eq!(TRACE_WIDTH, trace_info.width());
        TenAcciAir {
            context: AirContext::new(trace_info, degrees, 11, options),
            result: pub_inputs,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();
        // expected state width is 10 field elements
        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        result[0] = are_equal(
            next[0],
            current[0]
                + current[1]
                + current[2]
                + current[3]
                + current[4]
                + current[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[1] = are_equal(
            next[1],
            next[0]
                + current[1]
                + current[2]
                + current[3]
                + current[4]
                + current[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        //TODO: Index is 2 but length is 2
        //There must be an error on the trace width or maybe the transition constraint degree
        result[2] = are_equal(
            next[2],
            next[0]
                + next[1]
                + current[2]
                + current[3]
                + current[4]
                + current[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[3] = are_equal(
            next[3],
            next[0]
                + next[1]
                + next[2]
                + current[3]
                + current[4]
                + current[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[4] = are_equal(
            next[4],
            next[0]
                + next[1]
                + next[2]
                + next[3]
                + current[4]
                + current[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[5] = are_equal(
            next[5],
            next[0]
                + next[1]
                + next[2]
                + next[3]
                + next[4]
                + current[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[6] = are_equal(
            next[6],
            next[0]
                + next[1]
                + next[2]
                + next[3]
                + next[4]
                + next[5]
                + current[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[7] = are_equal(
            next[7],
            next[0]
                + next[1]
                + next[2]
                + next[3]
                + next[4]
                + next[5]
                + next[6]
                + current[7]
                + current[8]
                + current[9],
        );
        result[8] = are_equal(
            next[8],
            next[0]
                + next[1]
                + next[2]
                + next[3]
                + next[4]
                + next[5]
                + next[6]
                + next[7]
                + current[8]
                + current[9],
        );
        result[9] = are_equal(
            next[9],
            next[0]
                + next[1]
                + next[2]
                + next[3]
                + next[4]
                + next[5]
                + next[6]
                + next[7]
                + next[8]
                + current[9],
        );
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        // a valid tenacci sequence should start wit two ones and the first 8
        // powers of two and terminate with the expected result
        let last_step = self.trace_length() - 1;
        let two = Self::BaseField::ONE + Self::BaseField::ONE;
        vec![
            Assertion::single(0, 0, Self::BaseField::ONE),
            Assertion::single(1, 0, Self::BaseField::ONE),
            Assertion::single(2, 0, two),
            Assertion::single(3, 0, two.exp(2)),
            Assertion::single(4, 0, two.exp(3)),
            Assertion::single(5, 0, two.exp(4)),
            Assertion::single(6, 0, two.exp(5)),
            Assertion::single(7, 0, two.exp(6)),
            Assertion::single(8, 0, two.exp(7)),
            Assertion::single(9, 0, two.exp(8)),
            Assertion::single(9, last_step, self.result),
        ]
    }
}
