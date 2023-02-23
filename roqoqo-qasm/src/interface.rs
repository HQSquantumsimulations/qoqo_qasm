// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

// Operations that are ignored by backend and do not throw an error
const ALLOWED_OPERATIONS: &[&str; 7] = &[
    "PragmaSleep",
    "PragmaGlobalPhase",
    "PragmaStopParallelBlock",
    "PragmaStopDecompositionBlock",
    "PragmaSetNumberOfMeasurements",
    "PragmaStartDecompositionBlock",
    "InputSymbolic",
];

// Operations that are ignored when looking for a QASM definition
const NO_DEFINITION_REQUIRED_OPERATIONS: &[&str; 9] = &[
    "SingleQubitGate",
    "DefinitionFloat",
    "DefinitionUsize",
    "DefinitionBit",
    "DefinitionComplex",
    "PragmaActiveReset",
    "PragmaConditional",
    "PragmaRepeatedMeasurement",
    "MeasureQubit",
];

/// Translate the qoqo circuit into QASM ouput.
///
/// The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
/// to QASM output (strings).
///
/// # Arguments
///
/// * `circuit` - The qoqo Circuit that is translated.
/// * `qubit_register_name` - Name of the quantum register used for the roqoqo address-space
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
/// let circuit: Vec<String> = call_circuit(&circuit, "q").unwrap();
///
/// let manual_circuit: Vec<String> = vec![
///     "creg ro[1];".to_string(),
///     "x q[0];".to_string(),
///     "measure q[0] -> ro[0];".to_string()
/// ];
///
/// assert_eq!(circuit, manual_circuit);
/// ```
pub fn call_circuit(
    circuit: &Circuit,
    qubit_register_name: &str,
) -> Result<Vec<String>, RoqoqoBackendError> {
    let mut str_circuit: Vec<String> = Vec::new();
    for op in circuit.iter() {
        str_circuit.push(call_operation(op, qubit_register_name)?);
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
pub fn call_operation(
    operation: &Operation,
    qubit_register_name: &str,
) -> Result<String, RoqoqoBackendError> {
    match operation {
        Operation::RotateZ(op) => Ok(format!(
            "rz({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::RotateX(op) => Ok(format!(
            "rx({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::RotateY(op) => Ok(format!(
            "ry({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::Hadamard(op) => Ok(format!("h {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliX(op) => Ok(format!("x {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliY(op) => Ok(format!("y {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliZ(op) => Ok(format!("z {}[{}];", qubit_register_name, op.qubit())),
        Operation::SGate(op) => Ok(format!("s {}[{}];", qubit_register_name, op.qubit())),
        Operation::TGate(op) => Ok(format!("t {}[{}];", qubit_register_name, op.qubit())),
        Operation::PhaseShiftState1(op) => Ok(format!(
            "p({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::SqrtPauliX(op) => Ok(format!("sx {}[{}];", qubit_register_name, op.qubit())),
        Operation::InvSqrtPauliX(op) => {
            Ok(format!("sxdg {}[{}];", qubit_register_name, op.qubit()))
        }
        Operation::CNOT(op) => Ok(format!(
            "cx {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::MolmerSorensenXX(op) => Ok(format!(
            "rxx(pi/2) {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::VariableMSXX(op) => Ok(format!(
            "rxx({}) {}[{}],{}[{}];",
            op.theta(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ControlledPauliY(op) => Ok(format!(
            "cy {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ControlledPauliZ(op) => Ok(format!(
            "cz {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ControlledPhaseShift(op) => Ok(format!(
            "cp({}) {}[{}],{}[{}];",
            op.theta(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::SWAP(op) => Ok(format!(
            "swap {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ISwap(op) => Ok(format!(
            "iswap {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::SqrtISwap(op) => Ok(format!(
            "siswap {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::InvSqrtISwap(op) => Ok(format!(
            "siswapdg {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::FSwap(op) => Ok(format!(
            "fswap {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::Fsim(op) => Ok(format!(
            "fsim({},{},{}) {}[{}],{}[{}];",
            op.t().float().unwrap(),
            op.u().float().unwrap(),
            op.delta().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::Qsim(op) => Ok(format!(
            "qsim({},{},{}) {}[{}],{}[{}];",
            op.x().float().unwrap(),
            op.y().float().unwrap(),
            op.z().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::PMInteraction(op) => Ok(format!(
            "pmint({}) {}[{}],{}[{}];",
            op.t().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::GivensRotation(op) => Ok(format!(
            "gvnsrot({},{}) {}[{}],{}[{}];",
            op.theta().float().unwrap(),
            op.phi().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::GivensRotationLittleEndian(op) => Ok(format!(
            "gvnsrotle({},{}) {}[{}],{}[{}];",
            op.theta().float().unwrap(),
            op.phi().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::XY(op) => Ok(format!(
            "xy({}) {}[{}],{}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::SpinInteraction(op) => Ok(format!(
            "spinint({},{},{}) {}[{}],{}[{}];",
            op.x().float().unwrap(),
            op.y().float().unwrap(),
            op.z().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::RotateXY(op) => Ok(format!(
            "rxy({},{}) {}[{}];",
            op.theta().float().unwrap(),
            op.phi().float().unwrap(),
            qubit_register_name,
            op.qubit(),
        )),
        Operation::PhaseShiftedControlledZ(op) => Ok(format!(
            "pscz({}) {}[{}],{}[{}];",
            op.phi().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::PhaseShiftedControlledPhase(op) => Ok(format!(
            "pscp({},{}) {}[{}],{}[{}];",
            op.theta().float().unwrap(),
            op.phi().float().unwrap(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::SingleQubitGate(op) => {
            let alpha = CalculatorComplex::new(op.alpha_r(), op.alpha_i());
            let beta = CalculatorComplex::new(op.beta_r(), op.beta_i());
            let theta: CalculatorFloat = alpha.norm().acos() * 2.0;
            let phi: CalculatorFloat = alpha.arg() * (-1.0) + beta.arg();
            let lamda: CalculatorFloat = alpha.arg() * (-1.0) - beta.arg();

            Ok(format!(
                "u3({:.15},{:.15},{:.15}) {}[{}];",
                theta.float().unwrap(),
                phi.float().unwrap(),
                lamda.float().unwrap(),
                qubit_register_name,
                op.qubit()
            ))
        }
        Operation::PragmaActiveReset(op) => {
            Ok(format!("reset {}[{}];", qubit_register_name, op.qubit(),))
        }
        Operation::PragmaConditional(op) => {
            // does not work if the internal circuit has another PragmaConditional
            let mut ite = op.circuit().iter().peekable();
            let mut data = "".to_string();
            while let Some(int_op) = ite.next() {
                if ite.peek().is_none() {
                    data.push_str(&format!(
                        "if({}[{}]==1) {}",
                        op.condition_register(),
                        op.condition_index(),
                        call_operation(int_op, qubit_register_name).unwrap()
                    ));
                } else {
                    data.push_str(&format!(
                        "if({}[{}]==1) {}\n",
                        op.condition_register(),
                        op.condition_index(),
                        call_operation(int_op, qubit_register_name).unwrap()
                    ));
                }
            }
            Ok(data)
        }
        Operation::PragmaRepeatedMeasurement(op) => match op.qubit_mapping() {
            None => Ok(format!(
                "measure {} -> {};",
                qubit_register_name,
                op.readout()
            )),
            Some(qm) => {
                let mut output_string = "".to_string();
                for (key, val) in qm.iter() {
                    output_string += format!(
                        "measure {}[{}] -> {}[{}];\n",
                        qubit_register_name,
                        key,
                        op.readout(),
                        val
                    )
                    .as_str();
                }
                Ok(output_string)
            }
        },
        Operation::MeasureQubit(op) => Ok(format!(
            "measure {}[{}] -> {}[{}];",
            qubit_register_name,
            op.qubit(),
            op.readout(),
            op.readout_index()
        )),
        Operation::DefinitionFloat(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        Operation::DefinitionUsize(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        Operation::DefinitionBit(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        Operation::DefinitionComplex(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
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

/// Outputs the QASM gate definition of many qoqo operations.
///
/// # Arguments
///
/// * `operation` - The roqoqo Operation to be defined.
///
/// # Returns
///
/// * `Ok(String)` - The gate QASM gate definition.
/// * `RoqoqoBackendError::OperationNotInBackend` - Operation not supported by QASM backend.
pub fn gate_definition(operation: &Operation) -> Result<String, RoqoqoBackendError> {
    match operation {
        // TODO: always in output: u1 u2 u3 rx ry rz cx
        // TODO: check how qoqo-qiskit would react to these rotations
        Operation::RotateX(_) => Ok(String::from(
            "gate rx(theta) a { u3(theta,-pi/2,pi/2) a; }"
        )),
        Operation::RotateY(_) => Ok(String::from(
            "gate ry(theta) a { u3(theta,0,0) a; }"
        )),
        Operation::RotateZ(_) => Ok(String::from(
            "gate rz(phi) a { u1(phi) a; }"
        )),
        Operation::PauliX(_) => Ok(String::from(
            "gate x a { u3(pi,0,pi) a; }"
        )),
        Operation::PauliY(_) => Ok(String::from(
            "gate y a { u3(pi,pi/2,pi/2) a; }"
        )),
        Operation::PauliZ(_) => Ok(String::from(
            "gate z a { u1(pi) a; }"
        )),
        Operation::SGate(_) => Ok(String::from(
            "gate s a { u1(pi/2) a; }"
        )),
        Operation::TGate(_) => Ok(String::from(
            "gate t a { u1(pi/4) a; }"
        )),
        Operation::Hadamard(_) => Ok(String::from(
            "gate h a { u2(0,pi) a; }"
        )),
        Operation::CNOT(_) => Ok(String::from(
            "gate cx c,t { CX c,t; }"
        )),
        Operation::PhaseShiftState1(_) => Ok(String::from(
            "gate p(lambda) q { U(0,0,lambda) q; }"
        )),
        Operation::SqrtPauliX(_) => Ok(String::from(
            "gate sx a { u1(-pi/2) a; u2(0,pi) a; u1(-pi/2) a; }"
        )),
        Operation::InvSqrtPauliX(_) => Ok(String::from(
            "gate sxdg a { u1(pi/2) a; u2(0,pi) a; u1(pi/2) a; }"
        )),
        Operation::MolmerSorensenXX(_) | Operation::VariableMSXX(_) => Ok(String::from(
            "gate rxx(theta) a,b { u3(pi/2,theta,0) a; u2(0,pi) b; cx a,b; u1(-theta) b; cx a,b; u2(0,pi) b; u2(-pi,pi-theta) a; }"
        )),
        Operation::ControlledPauliY(_) => Ok(String::from(
            "gate cy a,b { u1(-pi/2) b; cx a,b; u1(pi/2) b; }"
        )),
        Operation::ControlledPauliZ(_) => Ok(String::from(
            "gate cz a,b { u2(0,pi) b; cx a,b; u2(0,pi) b; }"
        )),
        Operation::ControlledPhaseShift(_) => Ok(String::from(
            "gate cp(lambda) a,b { U(0,0,lambda/2) a; cx a,b; U(0,0,-lambda/2) b; cx a,b; U(0,0,lambda/2) b; }"
        )),
        Operation::SWAP(_) => Ok(String::from(
            "gate swap a,b { cx a,b; cx b,a; cx a,b; }"
        )),
        Operation::ISwap(_) => Ok(String::from(
            "gate iswap a,b { rx(pi/2) a; cx a,b; rx(-pi/2) a; ry(-pi/2) b; cx a,b; rx(-pi/2) a; }"
        )),
        Operation::SqrtISwap(_) => Ok(String::from(
            "gate siswap a,b { rx(pi/2) a; cx a,b; rx(-pi/4) a; ry(-pi/4) b; cx a,b; rx(-pi/2) a; }"
        )),
        Operation::InvSqrtISwap(_) => Ok(String::from(
            "gate siswapdg a,b { rx(pi/2) a; cx a,b; rx(pi/4) a; ry(pi/4) b; cx a,b; rx(-pi/2) a; }"
        )),
        Operation::FSwap(_) => Ok(String::from(
            "gate fswap a,b { rz(-pi/2) a; rz(-pi/2) b; rx(pi/2) a; cx a,b; rx(-pi/2) a; ry(-pi/2) b; cx a,b; rx(-pi/2) a; }"
        )),
        Operation::Fsim(_) => Ok(String::from(
            "gate fsim(t,u,delta) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; ry(-t+delta+pi/2) a; rx(pi) a; ry(-pi/2) b; rz((u-pi)/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(pi) a; ry(t+delta+pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(-pi/2) b; rx(-pi/2) b; rz((-u-pi)/2) a; rz((-u-pi)/2) b; }"
        )),
        Operation::PMInteraction(_) => Ok(String::from(
            "gate pmint(theta) a,b { rx(pi/2) a; cx a,b; rx(theta) a; ry(theta) b; cx a,b; rx(-pi/2) a; }"
        )),
        Operation::GivensRotation(_) => Ok(String::from(
            "gate gvnsrot(theta,phi) a,b { rz(phi+pi/2) b; rx(pi/2) a; cx a,b; rx(-theta) a; ry(-theta) b; cx a,b; rx(-pi/2) a; rz(-pi/2) b; }"
        )),
        Operation::GivensRotationLittleEndian(_) => Ok(String::from(
            "gate gvnsrotle(theta,phi) a,b { rz(-pi/2) a; rx(pi/2) a; cx a,b; rx(-theta) a; ry(-theta) b; cx a,b; rx(-pi/2) a; rz(phi+pi/2) a; }"
        )),
        Operation::Qsim(_) => Ok(String::from(
            "gate qsim(xc,yc,zc) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; ry(-2*xc+pi/2) a; rx(pi) a; ry(-pi/2) b; rz(2*zc-pi) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(pi) a; ry(2*yc+pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(-pi/2) b; rx(-pi/2) b; }"
        )),
        Operation::XY(_) => Ok(String::from(
            "gate xy(theta) a,b { rx(pi/2) a; cx a,b; rx(-theta/2) a; ry(-theta/2) b; cx a,b; rx(-pi/2) a; }"
        )),
        Operation::SpinInteraction(_) => Ok(String::from(
            "gate spinint(xc,yc,zc) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; ry(-2*xc) a; rx(pi) a; ry(-pi/2) b; rz(2*zc-pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(pi) a; ry(2*yc+pi) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(-pi/2) b; rx(-pi/2) b; }"
        )),
        Operation::PhaseShiftedControlledZ(_) => Ok(String::from(
            "gate pscz(phi) a,b { rz(pi/2) a; rz(pi/2) b; ry(pi/2) b; cx a,b; rx(-pi/2) b; rz(-pi/2) a; ry(-pi/2) b; rz(phi) a; rz(phi) b; }"
        )),
        Operation::PhaseShiftedControlledPhase(_) => Ok(String::from(
            "gate pscp(theta,phi) a,b { rz(theta/2) a; rz(theta/2) b; cx a,b; rz(-theta/2) b; cx a,b; rz(phi) a; rz(phi) b; }"
        )),
        Operation::RotateXY(_) => Ok(String::from(
            "gate rxy(theta,phi) q { u3(theta,phi-pi/2,pi/2-phi) q; }"
        )),
        _ => {
            if NO_DEFINITION_REQUIRED_OPERATIONS.contains(&operation.hqslang()) || ALLOWED_OPERATIONS.contains(&operation.hqslang()) {
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
