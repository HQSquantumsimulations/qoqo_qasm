// Copyright © 2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
//
//! Testing the roqoqo-qasm Interface

use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use roqoqo_qasm::{call_circuit, call_operation};
use std::collections::HashMap;
use std::f64::consts::PI;
use test_case::test_case;

/// Test that all operations return the correct String
#[test_case(Operation::from(PauliX::new(0)), "x q[0]"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "y q[0]"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)), "z q[0]"; "PauliZ")]
#[test_case(Operation::from(Hadamard::new(0)), "h q[0]"; "Hadamard")]
#[test_case(Operation::from(SGate::new(0)), "s q[0]"; "SGate")]
#[test_case(Operation::from(TGate::new(0)), "t q[0]"; "TGate")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(-PI))), "rx(-3.141592653589793) q[0]"; "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(-PI))), "ry(-3.141592653589793) q[0]"; "RotateY")]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(-PI))), "rz(-3.141592653589793) q[0]"; "RotateZ")]
#[test_case(Operation::from(SqrtPauliX::new(0)), "rx(pi/2) q[0]"; "SqrtPauliX")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), "rxx(pi/2) q[0],q[1]"; "MolmerSorensenXX")]
#[test_case(Operation::from(CNOT::new(0, 1)), "cx q[0],q[1]"; "CNOT")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "cy q[0],q[1]"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), "cz q[0],q[1]"; "ControlledPauliZ")]
#[test_case(Operation::from(SingleQubitGate::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), "u3(0.000000000000000,0.000000000000000,0.000000000000000) q[0]"; "SingleQubitGate")]
#[test_case(Operation::from(PragmaRepeatedMeasurement::new("ro".to_string(), Some(HashMap::new()), 1)), "measure q -> ro"; "PragmaRepeatedMeasurement")]
#[test_case(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)), "measure q[0] -> ro[0]"; "MeasureQubit")]
#[test_case(Operation::from(DefinitionFloat::new("ro".to_string(), 1, true)), "creg ro[1]"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionUsize::new("ro".to_string(), 1, true)), "creg ro[1]"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new("ro".to_string(), 1, true)), "creg ro[1]"; "DefinitionBit")]
#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 1, true)), "creg ro[1]"; "DefinitionComplex")]
#[test_case(Operation::from(InputSymbolic::new("other".to_string(), 0.0)), ""; "InputSymbolic")]
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(20, "ro".to_string())), ""; "PragmaSetNumberOfMeasurements")]
fn test_call_operation(operation: Operation, converted: &str) {
    assert_eq!(call_operation(&operation).unwrap(), converted.to_string())
}

/// Test that non-included gates return an error
#[test]
fn test_call_operation_error() {
    let operation = Operation::from(VariableMSXX::new(1, 0, CalculatorFloat::from(0.0)));
    assert_eq!(
        call_operation(&operation),
        Err(RoqoqoBackendError::OperationNotInBackend {
            backend: "QASM",
            hqslang: "VariableMSXX"
        })
    );
}

/// Test that a circuit can be correctly translated
#[test]
fn test_call_circuit() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 1, true);
    circuit += PauliX::new(0);
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);

    let mut qasm_circ: Vec<String> = Vec::new();
    qasm_circ.push("creg ro[1]".to_string());
    qasm_circ.push("x q[0]".to_string());
    qasm_circ.push("measure q[0] -> ro[0]".to_string());

    assert_eq!(call_circuit(&circuit).unwrap(), qasm_circ);
}
