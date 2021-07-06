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
//! The roqoqo-qasm Interface translates qoqo operations and circuits to QASM operations via the interface.

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::Circuit;
use roqoqo::RoqoqoBackendError;

// Pragma operations that are ignored by backend and do not throw an error
const ALLOWED_OPERATIONS: &[&str; 2] = &["PragmaSetNumberOfMeasurements", "InputSymbolic"];

/// Translate the qoqo circuit into QASM ouput.
///
/// The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
/// to QASM output (strings).
///
/// # Arguments
///
/// * `circuit` - The qoqo Circuit that is translated.
///
/// # Returns
///
/// * `Ok(Vec<&str>)` - Vector containing converted operations as strings.
/// * `Err(RoqoqoBackendError)` - Operation not supported by QASM backend.
///
/// # Example
/// ```
/// use roqoqo::{Circuit, operations::{DefinitionBit, PauliX, MeasureQubit}};
/// use roqoqo_qasm::call_circuit;
/// use std::collections::HashMap;
///
/// let mut circuit = Circuit::new();
/// circuit += DefinitionBit::new("ro".to_string(), 1, true);
/// circuit += PauliX::new(0);
/// circuit += MeasureQubit::new(0, "ro".to_string(), 0);
/// let circuit: Vec<String> = call_circuit(&circuit).unwrap();
///
/// let manual_circuit: Vec<String> = vec![
///     "creg ro[1]".to_string(),
///     "x q[0]".to_string(),
///     "measure q[0] -> ro[0]".to_string()
/// ];
///
/// assert_eq!(circuit, manual_circuit);
/// ```
///
pub fn call_circuit(circuit: &Circuit) -> Result<Vec<String>, RoqoqoBackendError> {
    let mut str_circuit: Vec<String> = Vec::new();
    for op in circuit.iter() {
        str_circuit.push(call_operation(op)?);
    }
    Ok(str_circuit)
}

/// Translates a qoqo operation to QASM (&str).
///
/// # Arguments
///
/// * `operation` - The qoqo Operation that is executed.
///
/// # Returns
///
/// * `Ok(&str)` - Converted operation in &str form.
/// * `Err(RoqoqoBackendError)` - Operation not supported by QASM backend.
///
pub fn call_operation(operation: &Operation) -> Result<String, RoqoqoBackendError> {
    match operation {
        Operation::RotateZ(op) => Ok(format!(
            "rz({}) q[{}]",
            op.theta().float().unwrap(),
            op.qubit()
        )),
        Operation::RotateX(op) => Ok(format!(
            "rx({}) q[{}]",
            op.theta().float().unwrap(),
            op.qubit()
        )),
        Operation::RotateY(op) => Ok(format!(
            "ry({}) q[{}]",
            op.theta().float().unwrap(),
            op.qubit()
        )),
        Operation::Hadamard(op) => Ok(format!("h q[{}]", op.qubit())),
        Operation::PauliX(op) => Ok(format!("x q[{}]", op.qubit())),
        Operation::PauliY(op) => Ok(format!("y q[{}]", op.qubit())),
        Operation::PauliZ(op) => Ok(format!("z q[{}]", op.qubit())),
        Operation::SGate(op) => Ok(format!("s q[{}]", op.qubit())),
        Operation::TGate(op) => Ok(format!("t q[{}]", op.qubit())),
        Operation::SqrtPauliX(op) => Ok(format!("rx(pi/2) q[{}]", op.qubit())),
        Operation::CNOT(op) => Ok(format!("cx q[{}],q[{}]", op.control(), op.target())),
        Operation::MolmerSorensenXX(op) => {
            Ok(format!("rxx(pi/2) q[{}],q[{}]", op.control(), op.target()))
        }
        Operation::ControlledPauliY(op) => Ok(format!("cy q[{}],q[{}]", op.control(), op.target())),
        Operation::ControlledPauliZ(op) => Ok(format!("cz q[{}],q[{}]", op.control(), op.target())),
        Operation::SingleQubitGate(op) => {
            let alpha = CalculatorComplex::new(op.alpha_r(), op.alpha_i());
            let beta = CalculatorComplex::new(op.beta_r(), op.beta_i());
            let theta: CalculatorFloat = alpha.norm().acos() * 2.0;
            let phi: CalculatorFloat = alpha.arg() * (-1.0) + beta.arg();
            let lamda: CalculatorFloat = alpha.arg() * (-1.0) - beta.arg();

            Ok(format!(
                "u3({:.15},{:.15},{:.15}) q[{}]",
                theta.float().unwrap(),
                phi.float().unwrap(),
                lamda.float().unwrap(),
                op.qubit()
            ))
        }
        Operation::PragmaRepeatedMeasurement(op) => Ok(format!("measure q -> {}", op.readout())),
        Operation::MeasureQubit(op) => Ok(format!(
            "measure q[{}] -> {}[{}]",
            op.qubit(),
            op.readout(),
            op.readout_index()
        )),
        Operation::DefinitionFloat(op) => Ok(format!("creg {}[{}]", op.name(), op.length())),
        Operation::DefinitionUsize(op) => Ok(format!("creg {}[{}]", op.name(), op.length())),
        Operation::DefinitionBit(op) => Ok(format!("creg {}[{}]", op.name(), op.length())),
        Operation::DefinitionComplex(op) => Ok(format!("creg {}[{}]", op.name(), op.length())),
        _ => {
            if ALLOWED_OPERATIONS.contains(&operation.hqslang()) {
                Ok("".to_string())
            } else {
                Err(RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang(),
                })
            }
        }
    }
}
