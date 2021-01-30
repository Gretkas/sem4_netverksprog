use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CalculationRequest {
    pub calculation: Vec<Calculation>,
    pub result: i32,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum Calculation {
    Number(i32),
    Operator(String),
}

impl CalculationRequest {
    pub fn calculate(&self) -> CalculationRequest {
        return CalculationRequest {
            calculation: self.calculation.to_owned(),
            result: CalculationRequest::complete_calculation(self.calculation.to_owned()),
        };
    }

    fn complete_calculation(calculation: Vec<Calculation>) -> i32 {
        let mut last_item: Calculation = Calculation::Operator(String::from("empty"));
        let mut result: i32 = 0;
        for element in calculation {
            match element {
                Calculation::Number(number) => {
                    match last_item {
                        Calculation::Operator(operation) => {
                            if operation.eq("+") {
                                result = result + number;
                            }
                            if operation.eq("-") {
                                result = result - number;
                            }
                            if operation.eq("empty") {
                                result = number;
                            }
                        }
                        _ => {}
                    }
                    last_item = Calculation::Number(number.to_owned());
                }
                Calculation::Operator(operation) => {
                    last_item = Calculation::Operator(operation.to_owned());
                }
            }
        }
        return result;
    }
}
