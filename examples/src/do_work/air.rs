use winterfell::{Air, AirContext, Assertion, TransitionConstraintDegree, math::{fields::f128::BaseElement, ToElements, FieldElement}};

// DO WORK AIR
// ================================================================================================

#[derive(Clone)]
pub struct PublicInputs {
    pub start: BaseElement,
    pub result: BaseElement,
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        vec![self.start, self.result]
    }
}

pub struct DoWorkAir {
    context: AirContext<BaseElement>,
    start: BaseElement,
    result: BaseElement,
}

impl Air for DoWorkAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(
        trace_info: winterfell::TraceInfo,
        pub_inputs: Self::PublicInputs,
        options: winterfell::ProofOptions,
    ) -> Self {
        assert_eq!(1, trace_info.width());
        let degrees = vec![TransitionConstraintDegree::new(3)];
        DoWorkAir {
            context: AirContext::new(trace_info, degrees, 2, options),
            start: pub_inputs.start,
            result: pub_inputs.result,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &winterfell::EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current_state = &frame.current()[0];
        let next_state = current_state.exp(3u32.into()) + E::from(42u32);
        result[0] = frame.next()[0] - next_state;
    }

    fn get_assertions(&self) -> Vec<winterfell::Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(0, 0, self.start),
            Assertion::single(0, last_step, self.result),
        ]
    }
}
