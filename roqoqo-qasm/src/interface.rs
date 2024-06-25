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

use crate::Qasm3Dialect;
use crate::QasmVersion;
use crate::VariableGatherer;

// Operations that are ignored by backend and do not throw an error
pub(crate) const ALLOWED_OPERATIONS: &[&str; 12] = &[
    "PragmaGetDensityMatrix",
    "PragmaGetOccupationProbability",
    "PragmaGetPauliProduct",
    "PragmaGetStateVector",
    "PragmaSleep",
    "PragmaSetNumberOfMeasurements",
    "PragmaStartDecompositionBlock",
    "PragmaStopDecompositionBlock",
    "PragmaStopParallelBlock",
    "InputSymbolic",
    "PragmaGlobalPhase",
    "GateDefinition",
];

// Operations that are ignored when looking for a QASM definition
pub(crate) const NO_DEFINITION_REQUIRED_OPERATIONS: &[&str; 12] = &[
    "SingleQubitGate",
    "DefinitionFloat",
    "DefinitionUsize",
    "DefinitionBit",
    "DefinitionComplex",
    "PragmaActiveReset",
    "PragmaConditional",
    "PragmaGlobalPhase",
    "PragmaRepeatedMeasurement",
    "MeasureQubit",
    "PragmaLoop",
    "CallDefinedGate",
];

/// Calls the parsing function of the VariableGatherer, if present.
///
/// # Arguments:
///
/// * `calculator_float` - The CalculatorFloat to gather from.
/// * `qasm_version` - The QASM version to use.
/// * `variable_gatherer` - Optional VariableParser to call.
///     
#[inline]
fn variable_gathering(
    calculator_float: &CalculatorFloat,
    qasm_version: QasmVersion,
    variable_gatherer: &mut Option<&mut VariableGatherer>,
) {
    if let Some(cp) = variable_gatherer {
        match qasm_version {
            QasmVersion::V3point0(_) => {
                let _ = cp.parse(calculator_float.to_string().as_str());
            }
            QasmVersion::V2point0 => (),
        }
    }
}

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
/// use roqoqo_qasm::{call_circuit, QasmVersion, Qasm3Dialect};
/// use std::collections::HashMap;
///
/// let mut circuit = Circuit::new();
/// circuit += DefinitionBit::new("ro".to_string(), 1, false);
/// circuit += PauliX::new(0);
/// circuit += MeasureQubit::new(0, "ro".to_string(), 0);
/// let circuit: Vec<String> = call_circuit(&circuit, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap();
///
/// let manual_circuit: Vec<String> = vec![
///     "bit[1] ro;".to_string(),
///     "x q[0];".to_string(),
///     "measure q[0] -> ro[0];".to_string()
/// ];
///
/// assert_eq!(circuit, manual_circuit);
/// ```
pub fn call_circuit(
    circuit: &Circuit,
    qubit_register_name: &str,
    qasm_version: QasmVersion,
) -> Result<Vec<String>, RoqoqoBackendError> {
    let mut str_circuit: Vec<String> = Vec::new();
    for op in circuit.iter() {
        str_circuit.push(call_operation(
            op,
            qubit_register_name,
            qasm_version,
            &mut None,
        )?);
    }
    Ok(str_circuit)
}

/// Translates a qoqo operation to QASM (&str).
///
/// # Arguments
///
/// * `operation` - The qoqo Operation that is executed.
/// * `qubtit_register_name` - Name of the quantum register used for the roqoqo address.
/// * `qasm_version` - The QASM version to use.
/// * `variable_gatherer` - Optional VariableParser to call.
///
/// # Returns
///
/// * `Ok(&str)` - Converted operation in &str form.
/// * `Err(RoqoqoBackendError)` - Operation not supported by QASM backend.
pub fn call_operation(
    operation: &Operation,
    qubit_register_name: &str,
    qasm_version: QasmVersion,
    variable_gatherer: &mut Option<&mut VariableGatherer>,
) -> Result<String, RoqoqoBackendError> {
    match operation {
        Operation::RotateZ(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            Ok(format!(
                "rz({}) {}[{}];",
                op.theta(),
                qubit_register_name,
                op.qubit()
            ))
        }
        Operation::RotateX(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            Ok(format!(
                "rx({}) {}[{}];",
                op.theta(),
                qubit_register_name,
                op.qubit()
            ))
        }
        Operation::RotateY(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            Ok(format!(
                "ry({}) {}[{}];",
                op.theta(),
                qubit_register_name,
                op.qubit()
            ))
        }
        Operation::Hadamard(op) => Ok(format!("h {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliX(op) => Ok(format!("x {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliY(op) => Ok(format!("y {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliZ(op) => Ok(format!("z {}[{}];", qubit_register_name, op.qubit())),
        Operation::SGate(op) => Ok(format!("s {}[{}];", qubit_register_name, op.qubit())),
        Operation::TGate(op) => Ok(format!("t {}[{}];", qubit_register_name, op.qubit())),
        Operation::PhaseShiftState1(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            match qasm_version {
                QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                    "phaseshift({}) {}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.qubit()
                )),
                _ => Ok(format!(
                    "p({}) {}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.qubit()
                )),
            }
        }
        Operation::SqrtPauliX(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Braket) => {
                Ok(format!("v {}[{}];", qubit_register_name, op.qubit()))
            }
            _ => Ok(format!("sx {}[{}];", qubit_register_name, op.qubit())),
        },
        Operation::InvSqrtPauliX(op) => {
            Ok(format!("sxdg {}[{}];", qubit_register_name, op.qubit()))
        }
        Operation::Identity(op) => Ok(format!("id {}[{}];", qubit_register_name, op.qubit())),
        Operation::CNOT(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                "cnot {}[{}],{}[{}];",
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            )),
            _ => Ok(format!(
                "cx {}[{}],{}[{}];",
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            )),
        },
        Operation::MolmerSorensenXX(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                "xx(pi/2) {}[{}],{}[{}];",
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            )),
            _ => Ok(format!(
                "rxx(pi/2) {}[{}],{}[{}];",
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            )),
        },
        Operation::VariableMSXX(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            match qasm_version {
                QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                    "xx({}) {}[{}],{}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.control(),
                    qubit_register_name,
                    op.target()
                )),
                _ => Ok(format!(
                    "rxx({}) {}[{}],{}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.control(),
                    qubit_register_name,
                    op.target()
                )),
            }
        }
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
        Operation::ControlledPhaseShift(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            match qasm_version {
                QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                    "cphaseshift({}) {}[{}],{}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.control(),
                    qubit_register_name,
                    op.target()
                )),
                _ => Ok(format!(
                    "cp({}) {}[{}],{}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.control(),
                    qubit_register_name,
                    op.target()
                )),
            }
        }
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
        Operation::Fsim(op) => {
            variable_gathering(op.t(), qasm_version, variable_gatherer);
            variable_gathering(op.u(), qasm_version, variable_gatherer);
            variable_gathering(op.delta(), qasm_version, variable_gatherer);
            Ok(format!(
                "fsim({},{},{}) {}[{}],{}[{}];",
                op.t(),
                op.u(),
                op.delta(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::Qsim(op) => {
            variable_gathering(op.x(), qasm_version, variable_gatherer);
            variable_gathering(op.y(), qasm_version, variable_gatherer);
            variable_gathering(op.z(), qasm_version, variable_gatherer);
            Ok(format!(
                "qsim({},{},{}) {}[{}],{}[{}];",
                op.x(),
                op.y(),
                op.z(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::PMInteraction(op) => {
            variable_gathering(op.t(), qasm_version, variable_gatherer);
            Ok(format!(
                "pmint({}) {}[{}],{}[{}];",
                op.t(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::GivensRotation(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            variable_gathering(op.phi(), qasm_version, variable_gatherer);
            Ok(format!(
                "gvnsrot({},{}) {}[{}],{}[{}];",
                op.theta(),
                op.phi(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::GivensRotationLittleEndian(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            variable_gathering(op.phi(), qasm_version, variable_gatherer);
            Ok(format!(
                "gvnsrotle({},{}) {}[{}],{}[{}];",
                op.theta(),
                op.phi(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::XY(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            Ok(format!(
                "xy({}) {}[{}],{}[{}];",
                op.theta(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::SpinInteraction(op) => {
            variable_gathering(op.x(), qasm_version, variable_gatherer);
            variable_gathering(op.y(), qasm_version, variable_gatherer);
            variable_gathering(op.z(), qasm_version, variable_gatherer);
            Ok(format!(
                "spinint({},{},{}) {}[{}],{}[{}];",
                op.x(),
                op.y(),
                op.z(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::RotateXY(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            variable_gathering(op.phi(), qasm_version, variable_gatherer);
            Ok(format!(
                "rxy({},{}) {}[{}];",
                op.theta(),
                op.phi(),
                qubit_register_name,
                op.qubit(),
            ))
        }
        Operation::PhaseShiftedControlledZ(op) => {
            variable_gathering(op.phi(), qasm_version, variable_gatherer);
            Ok(format!(
                "pscz({}) {}[{}],{}[{}];",
                op.phi(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::PhaseShiftedControlledPhase(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            variable_gathering(op.phi(), qasm_version, variable_gatherer);
            Ok(format!(
                "pscp({},{}) {}[{}],{}[{}];",
                op.theta(),
                op.phi(),
                qubit_register_name,
                op.control(),
                qubit_register_name,
                op.target()
            ))
        }
        Operation::GPi(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            match qasm_version {
                QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                    "gpi({}) {}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.qubit()
                )),
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
        Operation::GPi2(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            match qasm_version {
                QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(format!(
                    "gpi2({}) {}[{}];",
                    op.theta(),
                    qubit_register_name,
                    op.qubit()
                )),
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
        Operation::SingleQubitGate(op) => {
            let alpha = CalculatorComplex::new(op.alpha_r(), op.alpha_i());
            let beta = CalculatorComplex::new(op.beta_r(), op.beta_i());
            let theta: CalculatorFloat = alpha.norm().acos() * 2.0;
            let phi: CalculatorFloat = alpha.arg() * (-1.0) + beta.arg();
            let lamda: CalculatorFloat = alpha.arg() * (-1.0) - beta.arg();

            Ok(format!(
                "u3({:.15},{:.15},{:.15}) {}[{}];",
                theta.float()?,
                phi.float()?,
                lamda.float()?,
                qubit_register_name,
                op.qubit()
            ))
        }
        Operation::Toffoli(op) => Ok(format!(
            "ccx {}[{}],{}[{}],{}[{}];",
            qubit_register_name,
            op.control_0(),
            qubit_register_name,
            op.control_1(),
            qubit_register_name,
            op.target(),
        )),
        Operation::ControlledControlledPauliZ(op) => Ok(format!(
            "ccz {}[{}],{}[{}],{}[{}];",
            qubit_register_name,
            op.control_0(),
            qubit_register_name,
            op.control_1(),
            qubit_register_name,
            op.target(),
        )),
        Operation::ControlledControlledPhaseShift(op) => {
            variable_gathering(op.theta(), qasm_version, variable_gatherer);
            Ok(format!(
                "ccp({}) {}[{}],{}[{}],{}[{}];",
                op.theta(),
                qubit_register_name,
                op.control_0(),
                qubit_register_name,
                op.control_1(),
                qubit_register_name,
                op.target(),
            ))
        }
        Operation::PragmaActiveReset(op) => {
            Ok(format!("reset {}[{}];", qubit_register_name, op.qubit(),))
        }
        Operation::PragmaBoostNoise(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {};",
                op.hqslang(),
                op.noise_coefficient(),
            )),
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
        },
        Operation::PragmaConditional(op) => match qasm_version {
            QasmVersion::V2point0 => {
                let mut ite = op.circuit().iter().peekable();
                let mut data = "".to_string();
                while let Some(int_op) = ite.next() {
                    if int_op.tags().contains(&"PragmaConditional") {
                        return Err(RoqoqoBackendError::GenericError { msg: "For OpenQASM 2.0 we cannot have nested PragmaConditional operations".to_string() });
                    }
                    if ite.peek().is_none() {
                        data.push_str(&format!(
                            "if({}[{}]==1) {}",
                            op.condition_register(),
                            op.condition_index(),
                            call_operation(
                                int_op,
                                qubit_register_name,
                                qasm_version,
                                variable_gatherer
                            )?
                        ));
                    } else {
                        data.push_str(&format!(
                            "if({}[{}]==1) {}\n",
                            op.condition_register(),
                            op.condition_index(),
                            call_operation(
                                int_op,
                                qubit_register_name,
                                qasm_version,
                                variable_gatherer
                            )?
                        ));
                    }
                }
                Ok(data)
            }
            QasmVersion::V3point0(_) => {
                let mut data = "".to_string();
                let circuit_vec =
                    match call_circuit(op.circuit(), qubit_register_name, qasm_version) {
                        Ok(vec_str) => vec_str,
                        Err(x) => return Err(x),
                    };
                data.push_str(&format!(
                    "if({}[{}]==1) {{\n",
                    op.condition_register(),
                    op.condition_index(),
                ));
                for string in circuit_vec {
                    data.push_str(string.as_str());
                }
                data.push('}');
                Ok(data)
            }
        },
        Operation::PragmaDamping(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {} {};",
                op.hqslang(),
                op.qubit(),
                op.gate_time(),
                op.rate()
            )),
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
        },
        Operation::PragmaDephasing(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {} {};",
                op.hqslang(),
                op.qubit(),
                op.gate_time(),
                op.rate()
            )),
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
        },
        Operation::PragmaDepolarising(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {} {};",
                op.hqslang(),
                op.qubit(),
                op.gate_time(),
                op.rate()
            )),
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
        },
        Operation::PragmaGeneralNoise(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {} {};",
                op.hqslang(),
                op.qubit(),
                op.gate_time(),
                op.rates()
            )),
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
        },
        Operation::PragmaGetDensityMatrix(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {};",
                op.hqslang(),
                op.readout(),
                op.circuit().clone().unwrap_or(Circuit::new())
            )),
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
        },
        Operation::PragmaGetOccupationProbability(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {};",
                op.hqslang(),
                op.readout(),
                op.circuit().clone().unwrap_or(Circuit::new())
            )),
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
        },
        Operation::PragmaGetPauliProduct(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {:?} {} {};",
                op.hqslang(),
                op.qubit_paulis(),
                op.readout(),
                op.circuit()
            )),
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
        },
        Operation::PragmaGetStateVector(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {};",
                op.hqslang(),
                op.readout(),
                op.circuit().clone().unwrap_or(Circuit::new())
            )),
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
        },
        Operation::PragmaGlobalPhase(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!("gphase {};", op.phase(),)),
            QasmVersion::V3point0(Qasm3Dialect::Vanilla) => Ok(format!("gphase {};", op.phase(),)),
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
        },
        Operation::PragmaLoop(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {};",
                op.hqslang(),
                op.repetitions(),
                op.circuit()
            )),
            QasmVersion::V3point0(Qasm3Dialect::Vanilla) => {
                let mut data = "".to_string();
                match op.repetitions() {
                    CalculatorFloat::Float(x) => {
                        data.push_str(format!("for uint i in [0:{x}] {{\n").as_str());
                        let circuit_vec = match call_circuit(op.circuit(), qubit_register_name, qasm_version) {
                            Ok(vec_str) => vec_str,
                            Err(x) => return Err(x)
                        };
                        for string in circuit_vec {
                            data.push_str(format!("    {string}").as_str());
                        }
                        data.push_str("\n}");
                        Ok(data)
                    },
                    CalculatorFloat::Str(x) => Err(RoqoqoBackendError::GenericError { msg: format!("Used PragmaLoop with a string {x} for repetitions and a qasm-version that is incompatible: {qasm_version:?}") })
                }
            }
            _ => {
                let mut data = "".to_string();
                match op.repetitions() {
                    CalculatorFloat::Float(x) => {
                        for _ in 0_usize..(*x as usize) {
                            let circuit_vec = match call_circuit(op.circuit(), qubit_register_name, qasm_version) {
                                Ok(vec_str) => vec_str,
                                Err(x) => return Err(x)
                            };
                            for string in circuit_vec {
                                data.push_str(string.as_str());
                                data.push('\n');
                            }
                        }
                        Ok(data)
                    },
                    CalculatorFloat::Str(x) => Err(RoqoqoBackendError::GenericError { msg: format!("Used PragmaLoop with a string {x} for repetitions and a qasm-version that is incompatible: {qasm_version:?}") })
                }
            }
        },
        Operation::PragmaOverrotation(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {:?} {} {};",
                op.hqslang(),
                op.gate_hqslang(),
                op.qubits(),
                op.amplitude(),
                op.variance()
            )),
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
        },
        Operation::PragmaRandomNoise(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {} {} {};",
                op.hqslang(),
                op.qubit(),
                op.gate_time(),
                op.depolarising_rate(),
                op.dephasing_rate()
            )),
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
        },
        Operation::PragmaRepeatGate(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {};",
                op.hqslang(),
                op.repetition_coefficient()
            )),
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
        },
        Operation::PragmaRepeatedMeasurement(op) => {
            let mut output_string = "".to_string();
            match op.qubit_mapping() {
                None => output_string.push_str(
                    format!("measure {} -> {};", qubit_register_name, op.readout()).as_str(),
                ),
                Some(qm) => {
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
                }
            }
            if qasm_version == QasmVersion::V3point0(Qasm3Dialect::Roqoqo) {
                output_string.push_str(
                    format!(
                        "\npragma roqoqo PragmaSetNumberOfMeasurements {} {};",
                        op.number_measurements(),
                        op.readout(),
                    )
                    .as_str(),
                );
            };
            Ok(output_string)
        }
        Operation::PragmaSetDensityMatrix(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {};",
                op.hqslang(),
                op.density_matrix()
            )),
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
        },
        Operation::PragmaSetNumberOfMeasurements(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {} {};",
                op.hqslang(),
                op.number_measurements(),
                op.readout()
            )),
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
        },
        Operation::PragmaSetStateVector(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {};",
                op.hqslang(),
                op.statevector()
            )),
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
        },
        Operation::PragmaSleep(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {:?} {};",
                op.hqslang(),
                op.qubits(),
                op.sleep_time()
            )),
            QasmVersion::V2point0 => {
                let mut output_string = "".to_string();
                for (ind, qbt) in op.qubits().iter().enumerate() {
                    output_string.push_str(
                        format!(
                            "pragmasleep({}) {}[{}];",
                            op.sleep_time(),
                            qubit_register_name,
                            qbt
                        )
                        .as_str(),
                    );
                    if ind != op.qubits().len() - 1 {
                        output_string.push('\n');
                    }
                }
                Ok(output_string)
            }
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
        },
        Operation::PragmaStartDecompositionBlock(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {:?} {:?};",
                op.hqslang(),
                op.qubits(),
                op.reordering_dictionary()
            )),
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
        },
        Operation::PragmaStopDecompositionBlock(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => {
                Ok(format!("pragma roqoqo {} {:?};", op.hqslang(), op.qubits()))
            }
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
        },
        Operation::PragmaStopParallelBlock(op) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo) => Ok(format!(
                "pragma roqoqo {} {:?} {};",
                op.hqslang(),
                op.qubits(),
                op.execution_time()
            )),
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
        },
        Operation::MeasureQubit(op) => Ok(format!(
            "measure {}[{}] -> {}[{}];",
            qubit_register_name,
            op.qubit(),
            op.readout(),
            op.readout_index()
        )),
        Operation::DefinitionFloat(op) => match qasm_version {
            QasmVersion::V2point0 => Ok(format!("creg {}[{}];", op.name(), op.length())),
            QasmVersion::V3point0(Qasm3Dialect::Braket) => {
                Ok(format!("float[{}] {};", op.length(), op.name(),))
            }
            QasmVersion::V3point0(_) => {
                if *op.is_output() {
                    Ok(format!("output float[{}] {};", op.length(), op.name(),))
                } else {
                    Ok(format!("float[{}] {};", op.length(), op.name(),))
                }
            }
        },
        Operation::DefinitionUsize(op) => match qasm_version {
            QasmVersion::V2point0 => Ok(format!("creg {}[{}];", op.name(), op.length())),
            QasmVersion::V3point0(Qasm3Dialect::Braket) => {
                Ok(format!("uint[{}] {};", op.length(), op.name(),))
            }
            QasmVersion::V3point0(_) => {
                if *op.is_output() {
                    Ok(format!("output uint[{}] {};", op.length(), op.name(),))
                } else {
                    Ok(format!("uint[{}] {};", op.length(), op.name(),))
                }
            }
        },
        Operation::DefinitionBit(op) => match qasm_version {
            QasmVersion::V2point0 => Ok(format!("creg {}[{}];", op.name(), op.length())),
            QasmVersion::V3point0(Qasm3Dialect::Braket) => {
                Ok(format!("bit[{}] {};", op.length(), op.name(),))
            }
            QasmVersion::V3point0(_) => {
                if *op.is_output() {
                    Ok(format!("output bit[{}] {};", op.length(), op.name(),))
                } else {
                    Ok(format!("bit[{}] {};", op.length(), op.name(),))
                }
            }
        },
        Operation::DefinitionComplex(op) => match qasm_version {
            QasmVersion::V2point0 => Ok(format!("creg {}[{}];", op.name(), op.length())),
            QasmVersion::V3point0(Qasm3Dialect::Braket) => {
                let mut data = "".to_string();
                data.push_str(&format!("float[{}] {}_re;\n", op.length(), op.name(),));
                data.push_str(&format!("float[{}] {}_im;", op.length(), op.name(),));
                Ok(data)
            }
            QasmVersion::V3point0(_) => {
                let mut data = "".to_string();
                if *op.is_output() {
                    data.push_str(&format!(
                        "output float[{}] {}_re;\n",
                        op.length(),
                        op.name(),
                    ));
                    data.push_str(&format!("output float[{}] {}_im;", op.length(), op.name(),));
                } else {
                    data.push_str(&format!("float[{}] {}_re;\n", op.length(), op.name(),));
                    data.push_str(&format!("float[{}] {}_im;", op.length(), op.name(),));
                }
                Ok(data)
            }
        },
        Operation::InputSymbolic(op) => match qasm_version {
            QasmVersion::V3point0(_) => Ok(format!("input float {};", op.name())),
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
        },
        Operation::InputBit(op) => match qasm_version {
            QasmVersion::V3point0(_) => {
                Ok(format!("{}[{}] = {};", op.name(), op.index(), op.value()))
            }
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
        },
        Operation::CallDefinedGate(op) => Ok(format!(
            "{}({}) {};",
            op.gate_name(),
            op.free_parameters()
                .iter()
                .map(|param| param.to_string())
                .collect::<Vec<String>>()
                .join(","),
            op.qubits()
                .iter()
                .map(|qubit| format!("{}[{}]", qubit_register_name, qubit))
                .collect::<Vec<String>>()
                .join(",")
        )),
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
pub fn gate_definition(
    operation: &Operation,
    qasm_version: QasmVersion,
) -> Result<String, RoqoqoBackendError> {
    match operation {
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
        Operation::CNOT(_) => match qasm_version {
            QasmVersion::V2point0 => Ok(String::from(
                "gate cx c,t { CX c,t; }"
            )),
            QasmVersion::V3point0(_) => Ok(String::from(
                "gate cx c,t { ctrl @ x c,t; }"
            ))
        },
        Operation::PhaseShiftState1(_) => Ok(String::from(
            "gate p(lambda) q { U(0,0,lambda) q; }"
        )),
        Operation::SqrtPauliX(_) => Ok(String::from(
            "gate sx a { u1(-pi/2) a; u2(0,pi) a; u1(-pi/2) a; }"
        )),
        Operation::InvSqrtPauliX(_) => Ok(String::from(
            "gate sxdg a { u1(pi/2) a; u2(0,pi) a; u1(pi/2) a; }"
        )),
        Operation::Identity(_) => Ok(String::from(
            "gate id a { U(0,0,0) a; }"
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
        Operation::Toffoli(_) => Ok(String::from(
            "gate ccx a,b,c { u2(0,pi) c; cx b,c; u1(-pi/4) c; cx a,c; u1(pi/4) c; cx b,c; u1(-pi/4) c; cx a,c; u1(pi/4) b; u1(pi/4) c; u2(0,pi) c; cx a,b; u1(pi/4) a; u1(-pi/4) b; cx a,b; }"
        )),
        Operation::ControlledControlledPauliZ(_) => Ok(String::from(
            "gate ccz a,b,c { U(0,0,pi/4) b; cx b,c; U(0,0,-pi/4) c; cx b,c; U(0,0,pi/4) c; cx a,b; U(0,0,-pi/4) b; cx b,c; U(0,0,pi/4) c; cx b,c; U(0,0,-pi/4) c; cx a,b; U(0,0,pi/4) a; cx a,c; U(0,0,-pi/4) c; cx a,c; U(0,0,pi/4) c; }"
        )),
        Operation::ControlledControlledPhaseShift(_) => Ok(String::from(
            "gate ccp(theta) a,b,c { U(0,0,theta/4) b; cx b,c; U(0,0,-theta/4) c; cx b,c; U(0,0,theta/4) c; cx a,b; U(0,0,-theta/4) b; cx b,c; U(0,0,theta/4) c; cx b,c; U(0,0,-theta/4) c; cx a,b; U(0,0,theta/4) a; cx a,c; U(0,0,-theta/4) c; cx a,c; U(0,0,theta/4) c; }"
        )),
        Operation::GPi(_) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(String::from(
                "gate gpi(theta) a { u3(pi,-pi/2,pi/2) a; u1(2*theta) a; gphase pi/2; }"
            )),
            _ => Err(RoqoqoBackendError::OperationNotInBackend {
                backend: "QASM",
                hqslang: operation.hqslang(),
            }),
        },
        Operation::GPi2(_) => match qasm_version {
            QasmVersion::V3point0(Qasm3Dialect::Braket) => Ok(String::from(
                "gate gpi2(theta) a { u1(-theta) a; u3(pi/2,-pi/2,pi/2) a; u1(theta) a; }"
            )),
            _ => Err(RoqoqoBackendError::OperationNotInBackend {
                backend: "QASM",
                hqslang: operation.hqslang(),
            }),
        },
        Operation::PragmaSleep(_) => Ok(String::from(
            "opaque pragmasleep(param) a;"
        )),
        Operation::GateDefinition(gate_definition) => {
            let mut definition_str = format!(
                "gate {}({}) {}\n{{\n",
                gate_definition.name(),
                gate_definition.free_parameters().join(","),
                gate_definition
                    .qubits()
                    .iter()
                    .map(|&qubit| format!("qb_{}", qubit).to_owned())
                    .collect::<Vec<String>>()
                    .join(",")
            );
            for operation in gate_definition.circuit().iter() {
                definition_str.push_str("    ");
                definition_str.push_str(&call_operation(
                    operation,
                    "replace_me",
                    qasm_version,
                    &mut None,
                )?);
                definition_str.push('\n');
            }
            definition_str.push('}');
            for qubit in gate_definition.qubits().iter() {
                definition_str = definition_str
                    .replace(&format!("replace_me[{}]", qubit), &format!("qb_{}", qubit));
            }
            Ok(definition_str)
        }
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
