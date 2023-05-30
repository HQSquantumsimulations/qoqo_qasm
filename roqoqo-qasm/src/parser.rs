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

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use qoqo_calculator::CalculatorFloat;

use roqoqo::operations::*;
use roqoqo::Circuit;

/// Pest Parser for QASM -> qoqo translation.
#[derive(Parser, Debug)]
#[grammar = "grammars/qasm2_0.pest"]
struct QoqoQASMParser;

/// Dispatch function for qoqo operations.
fn gate_dispatch(name: &str, params: &[f64], qubits: &[usize]) -> Option<Operation> {
    match name {
        "rz" => Some(Operation::from(RotateZ::new(
            qubits[0],
            CalculatorFloat::from(params[0]),
        ))),
        "ry" => Some(Operation::from(RotateY::new(
            qubits[0],
            CalculatorFloat::from(params[0]),
        ))),
        "rx" => Some(Operation::from(RotateX::new(
            qubits[0],
            CalculatorFloat::from(params[0]),
        ))),
        "h" => Some(Operation::from(Hadamard::new(qubits[0]))),
        "x" => Some(Operation::from(PauliX::new(qubits[0]))),
        "y" => Some(Operation::from(PauliY::new(qubits[0]))),
        "z" => Some(Operation::from(PauliZ::new(qubits[0]))),
        "s" => Some(Operation::from(SGate::new(qubits[0]))),
        "t" => Some(Operation::from(TGate::new(qubits[0]))),
        "p" => Some(Operation::from(PhaseShiftState1::new(
            qubits[0],
            CalculatorFloat::from(params[0]),
        ))),
        "sx" => Some(Operation::from(SqrtPauliX::new(qubits[0]))),
        "sxdg" => Some(Operation::from(InvSqrtPauliX::new(qubits[0]))),
        "cx" => Some(Operation::from(CNOT::new(qubits[0], qubits[1]))),
        // MolmerSorensenXX
        "rxx" => Some(Operation::from(VariableMSXX::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
        ))),
        "cy" => Some(Operation::from(ControlledPauliY::new(qubits[0], qubits[1]))),
        "cz" => Some(Operation::from(ControlledPauliZ::new(qubits[0], qubits[1]))),
        "cp" => Some(Operation::from(ControlledPhaseShift::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
        ))),
        "swap" => Some(Operation::from(SWAP::new(qubits[0], qubits[1]))),
        "iswap" => Some(Operation::from(ISwap::new(qubits[0], qubits[1]))),
        "siswap" => Some(Operation::from(SqrtISwap::new(qubits[0], qubits[1]))),
        "siswapdg" => Some(Operation::from(InvSqrtISwap::new(qubits[0], qubits[1]))),
        "fswap" => Some(Operation::from(FSwap::new(qubits[0], qubits[1]))),
        "fsim" => Some(Operation::from(Fsim::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
            CalculatorFloat::from(params[2]),
        ))),
        "qsim" => Some(Operation::from(Qsim::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
            CalculatorFloat::from(params[2]),
        ))),
        "pmint" => Some(Operation::from(PMInteraction::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
        ))),
        "gvnsrot" => Some(Operation::from(GivensRotation::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
        ))),
        "gvnsrotle" => Some(Operation::from(GivensRotationLittleEndian::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
        ))),
        "xy" => Some(Operation::from(XY::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
        ))),
        "spintint" => Some(Operation::from(SpinInteraction::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
            CalculatorFloat::from(params[2]),
        ))),
        "rxy" => Some(Operation::from(RotateXY::new(
            qubits[0],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
        ))),
        "pscz" => Some(Operation::from(PhaseShiftedControlledZ::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
        ))),
        "pscp" => Some(Operation::from(PhaseShiftedControlledPhase::new(
            qubits[0],
            qubits[1],
            CalculatorFloat::from(params[0]),
            CalculatorFloat::from(params[1]),
        ))),
        // "u3" => {
        //     let alpha_r =
        //     Some(Operation::from(SingleQubitGate::new(qubits[0], CalculatorFloat::from(params[0]), CalculatorFloat::from(params[1]), CalculatorFloat::from(params[2]), CalculatorFloat::from(params[3]))))
        // },
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
                let mut params: Vec<f64> = vec![];
                let mut qubits: Vec<usize> = vec![];
                for pair in inner_pairs.clone() {
                    match pair.as_rule() {
                        Rule::parameter_list => {
                            let params_list = inner_pairs.next().unwrap().into_inner().clone();
                            for real in params_list {
                                let real_f64 = real.as_str().parse::<f64>().unwrap();
                                params.push(real_f64);
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
pub fn qasm_file_to_circuit(file: File) -> Result<Circuit, Box<dyn std::error::Error>> {
    let unparsed_file = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap() + "\n")
        .collect::<String>();

    let circuit: Circuit = parse_qasm_file(&unparsed_file)?;

    Ok(circuit)
}
