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
//! The roqoqo-qasm Parser translates qasm files in Qoqo Circuit instances.

use num_complex::Complex64;
use qoqo_calculator::Calculator;
use roqoqo::RoqoqoBackendError;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::Circuit;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

/// Pest Parser for QASM -> qoqo translation.
#[derive(Parser, Debug)]
#[grammar = "grammars/qasm2_0.pest"]
struct QoqoQASMParser;

/// Dispatch function for qoqo operations.
fn gate_dispatch(name: &str, params: &[String], qubits: &[usize]) -> Option<Operation> {
    match name {
        "rz" => Some(Operation::from(RotateZ::new(
            qubits[0],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "ry" => Some(Operation::from(RotateY::new(
            qubits[0],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "rx" => Some(Operation::from(RotateX::new(
            qubits[0],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "h" => Some(Operation::from(Hadamard::new(qubits[0]))),
        "x" => Some(Operation::from(PauliX::new(qubits[0]))),
        "y" => Some(Operation::from(PauliY::new(qubits[0]))),
        "z" => Some(Operation::from(PauliZ::new(qubits[0]))),
        "s" => Some(Operation::from(SGate::new(qubits[0]))),
        "t" => Some(Operation::from(TGate::new(qubits[0]))),
        "p" => Some(Operation::from(PhaseShiftState1::new(
            qubits[0],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "sx" => Some(Operation::from(SqrtPauliX::new(qubits[0]))),
        "sxdg" => Some(Operation::from(InvSqrtPauliX::new(qubits[0]))),
        "cx" => Some(Operation::from(CNOT::new(qubits[0], qubits[1]))),
        "rxx" => {
            if let Ok(float) = CalculatorFloat::from(params[0].clone()).float() {
                if is_close(float.into(), CalculatorFloat::PI.float().unwrap().into()) {
                    return Some(Operation::from(MolmerSorensenXX::new(qubits[0], qubits[1])));
                }
            }
            Some(Operation::from(VariableMSXX::new(
                qubits[0],
                qubits[1],
                CalculatorFloat::from(params[0].clone()),
            )))
        }
        "cy" => Some(Operation::from(ControlledPauliY::new(qubits[0], qubits[1]))),
        "cz" => Some(Operation::from(ControlledPauliZ::new(qubits[0], qubits[1]))),
        "cp" => Some(Operation::from(ControlledPhaseShift::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "swap" => Some(Operation::from(SWAP::new(qubits[0], qubits[1]))),
        "iswap" => Some(Operation::from(ISwap::new(qubits[0], qubits[1]))),
        "siswap" => Some(Operation::from(SqrtISwap::new(qubits[0], qubits[1]))),
        "siswapdg" => Some(Operation::from(InvSqrtISwap::new(qubits[0], qubits[1]))),
        "fswap" => Some(Operation::from(FSwap::new(qubits[0], qubits[1]))),
        "fsim" => Some(Operation::from(Fsim::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
            CalculatorFloat::from(params[2].clone()),
        ))),
        "qsim" => Some(Operation::from(Qsim::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
            CalculatorFloat::from(params[2].clone()),
        ))),
        "pmint" => Some(Operation::from(PMInteraction::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "gvnsrot" => Some(Operation::from(GivensRotation::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
        ))),
        "gvnsrotle" => Some(Operation::from(GivensRotationLittleEndian::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
        ))),
        "xy" => Some(Operation::from(XY::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "spintint" => Some(Operation::from(SpinInteraction::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
            CalculatorFloat::from(params[2].clone()),
        ))),
        "rxy" => Some(Operation::from(RotateXY::new(
            qubits[0],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
        ))),
        "pscz" => Some(Operation::from(PhaseShiftedControlledZ::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
        ))),
        "pscp" => Some(Operation::from(PhaseShiftedControlledPhase::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0].clone()),
            CalculatorFloat::from(params[1].clone()),
        ))),
        "u3" => {
            let theta = CalculatorFloat::from(params[0].clone());
            let phi = CalculatorFloat::from(params[1].clone());
            let lambda = CalculatorFloat::from(params[2].clone());
            let alpha_r =
                ((phi.clone() + lambda.clone()) / 2.0).cos() * (theta.clone() / 2.0).cos();
            let alpha_i =
                (-(phi.clone() + lambda.clone()) / 2.0).sin() * (theta.clone() / 2.0).cos();
            let beta_r = ((phi.clone() - lambda.clone()) / 2.0).cos() * (theta.clone() / 2.0).sin();
            let beta_i = ((phi - lambda) / 2.0).sin() * (theta / 2.0).sin();
            Some(Operation::from(SingleQubitGate::new(
                qubits[0],
                alpha_r,
                alpha_i,
                beta_r,
                beta_i,
                CalculatorFloat::ZERO,
            )))
        }
        "ccx" => Some(Operation::from(Toffoli::new(
            qubits[0], qubits[1], qubits[2],
        ))),
        "ccz" => Some(Operation::from(ControlledControlledPauliZ::new(
            qubits[0], qubits[1], qubits[2],
        ))),
        "ccp" => Some(Operation::from(ControlledControlledPhaseShift::new(
            qubits[0],
            qubits[1],
            qubits[2],
            CalculatorFloat::from(params[0].clone()),
        ))),
        _ => None,
    }
}

/// Main parse function method.
fn parse_qasm_file(file: &str) -> Result<Circuit, Box<Error<Rule>>> {
    let pairs = QoqoQASMParser::parse(Rule::openqasm, file)?;
    let mut circuit = Circuit::new();

    /// The parsing works like an AST traversal. The structure is defined by the grammar.
    ///     - pair.as_rule() represents the rule itself, to get into the inner ones, `.into_inner()` is called
    ///     - from the new inner instance we can further move to the right in the rule by calling `.next().unwrap()[.as_str()]`
    fn parse_single_rule(pair: Pair<Rule>) -> Option<Operation> {
        match pair.as_rule() {
            Rule::c_decl => {
                let mut inner_pairs = pair.into_inner();
                let id = inner_pairs.next().unwrap().as_str();
                let integer = inner_pairs
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                Some(Operation::from(DefinitionBit::new(
                    id.to_string(),
                    integer,
                    true,
                )))
            }
            Rule::gate => {
                let mut inner_pairs = pair.into_inner();
                let id = inner_pairs.next().unwrap().as_str();
                let mut params: Vec<String> = vec![];
                let mut qubits: Vec<usize> = vec![];
                for pair in inner_pairs.clone() {
                    match pair.as_rule() {
                        Rule::parameter_list => {
                            let params_list = inner_pairs.next().unwrap().into_inner().clone();
                            for param in params_list {
                                // Handle 'pi' constant and math functions renames (Calculator)
                                let mut param_str =
                                    param.as_str().replace("pi", "3.141592653589793");
                                param_str = param_str.replace("ln", "log");
                                // Parse the mathematical expression
                                let calc = Calculator::new();
                                let parsed = calc.parse_str(&param_str).unwrap();
                                // Pass the parsed expression (now float) as String
                                params.push(parsed.to_string());
                            }
                        }
                        Rule::qubit_list => {
                            let qbt_list = inner_pairs.next().unwrap().into_inner().clone();
                            for qbt_rule in qbt_list {
                                let mut inner_pairs = qbt_rule.into_inner();
                                let _id = inner_pairs.next().unwrap().as_str();
                                let integer = inner_pairs
                                    .next()
                                    .unwrap()
                                    .as_str()
                                    .parse::<usize>()
                                    .unwrap();
                                qubits.push(integer);
                            }
                        }
                        _ => continue,
                    }
                }
                gate_dispatch(id, &params, &qubits)
            }
            Rule::measurement => {
                let mut inner_pairs = pair.into_inner();
                let mut first_argument = inner_pairs.next().unwrap().into_inner();
                let _first_id = first_argument.next().unwrap().as_str();
                let first_integer = first_argument.next().unwrap().as_str();
                let mut second_argument = inner_pairs.next().unwrap().into_inner();
                let second_id = second_argument.next().unwrap().as_str();
                let second_integer = second_argument.next().unwrap().as_str();
                Some(Operation::from(MeasureQubit::new(
                    first_integer.parse::<usize>().unwrap(),
                    second_id.to_string(),
                    second_integer.parse::<usize>().unwrap(),
                )))
            }
            Rule::reset => {
                let mut inner_pairs = pair.into_inner();
                let mut first_argument = inner_pairs.next().unwrap().into_inner();
                let _first_id = first_argument.next().unwrap().as_str();
                let first_integer = first_argument.next().unwrap().as_str();
                Some(Operation::from(PragmaActiveReset::new(
                    first_integer.parse::<usize>().unwrap(),
                )))
            }
            _ => None,
        }
    }

    for pair in pairs {
        if let Some(op) = parse_single_rule(pair) {
            circuit.add_operation(op);
        }
    }

    Ok(circuit)
}

/// Translates a QASM file into a qoqo Circuit instance.
///
/// # Arguments
///
/// * `file` - The '.qasm' file to translate.
///
/// # Returns
///
/// * `Circuit` - The translated qoqo Circuit.
/// * `RoqoqoBackendError::GenericError` - Error encountered while parsing.
pub fn file_to_circuit(file: File) -> Result<Circuit, RoqoqoBackendError> {
    let unparsed_file = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap() + "\n")
        .collect::<String>();

    parse_qasm_file(&unparsed_file).map_err(|x| RoqoqoBackendError::GenericError {
        msg: format!("Error during conversion: {}", x),
    })
}

/// Translates a QASM string into a qoqo Circuit instance.
///
/// # Arguments
///
/// * `input` - The QASM string to translate.
///
/// # Returns
///
/// * `Circuit` - The translated qoqo Circuit.
/// * `RoqoqoBackendError::GenericError` - Error encountered while parsing.
pub fn string_to_circuit(input: &str) -> Result<Circuit, RoqoqoBackendError> {
    parse_qasm_file(input).map_err(|x| RoqoqoBackendError::GenericError {
        msg: format!("Error during conversion: {}", x),
    })
}

// helper function
fn is_close(a: Complex64, b: Complex64) -> bool {
    (a - b).norm() < 1e-10
}
