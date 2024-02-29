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
//! Testing the roqoqo-qasm Parser

use std::convert::TryInto;
use std::fs::File;

use num_complex::Complex64;
use roqoqo::operations::*;
use roqoqo::Circuit;

use roqoqo_qasm::file_to_circuit;

// helper function
fn is_close(a: Complex64, b: Complex64) -> bool {
    (a - b).norm() < 1e-10
}

/// Test basic file
#[test]
fn test_basic_file() {
    let file = File::open(std::env::current_dir().unwrap().join("tests/input.qasm")).unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 2, true);
    circuit_qoqo += PauliX::new(0);
    circuit_qoqo += Hadamard::new(1);
    circuit_qoqo += RotateX::new(2, 2.3.into());
    circuit_qoqo += CNOT::new(0, 1);
    circuit_qoqo += MeasureQubit::new(0, "c".into(), 0);

    assert_eq!(circuit_from_file, circuit_qoqo);
}

/// Test all implemented qoqo gates
#[test]
fn test_qoqo_gates() {
    let file = File::open(std::env::current_dir().unwrap().join("tests/gates.qasm")).unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 2, true);
    circuit_qoqo += RotateZ::new(0, 0.2.into());
    circuit_qoqo += RotateY::new(1, 0.3.into());
    circuit_qoqo += RotateX::new(2, 2.1.into());
    circuit_qoqo += Hadamard::new(0);
    circuit_qoqo += PauliX::new(2);
    circuit_qoqo += PauliY::new(1);
    circuit_qoqo += PauliZ::new(0);
    circuit_qoqo += SGate::new(0);
    circuit_qoqo += TGate::new(1);
    circuit_qoqo += PhaseShiftState1::new(2, 0.6.into());
    circuit_qoqo += SqrtPauliX::new(1);
    circuit_qoqo += InvSqrtPauliX::new(0);
    circuit_qoqo += CNOT::new(0, 1);
    circuit_qoqo += MolmerSorensenXX::new(1, 2);
    circuit_qoqo += VariableMSXX::new(0, 2, 0.7.into());
    circuit_qoqo += ControlledPauliY::new(0, 1);
    circuit_qoqo += ControlledPauliZ::new(1, 2);
    circuit_qoqo += ControlledPhaseShift::new(0, 2, 0.5.into());
    circuit_qoqo += SWAP::new(1, 2);
    circuit_qoqo += ISwap::new(0, 1);
    circuit_qoqo += SqrtISwap::new(1, 2);
    circuit_qoqo += InvSqrtISwap::new(0, 2);
    circuit_qoqo += FSwap::new(0, 1);
    circuit_qoqo += Fsim::new(1, 2, 0.1.into(), 0.2.into(), 0.3.into());
    circuit_qoqo += Qsim::new(0, 2, 1.0.into(), 1.1.into(), 1.2.into());
    circuit_qoqo += PMInteraction::new(1, 2, 0.8.into());
    circuit_qoqo += GivensRotation::new(1, 2, 0.2.into(), 0.9.into());
    circuit_qoqo += GivensRotationLittleEndian::new(0, 2, 0.0.into(), 0.8.into());
    circuit_qoqo += XY::new(1, 2, 0.5.into());
    circuit_qoqo += SpinInteraction::new(0, 2, 0.5.into(), 0.6.into(), 0.7.into());
    circuit_qoqo += RotateXY::new(0, 0.3.into(), 0.9.into());
    circuit_qoqo += PhaseShiftedControlledZ::new(0, 2, 0.3.into());
    circuit_qoqo += PhaseShiftedControlledPhase::new(0, 1, 1.0.into(), 1.9.into());
    circuit_qoqo += Toffoli::new(0, 2, 1);
    circuit_qoqo += ControlledControlledPauliZ::new(2, 1, 0);
    circuit_qoqo += ControlledControlledPhaseShift::new(1, 0, 2, 0.3.into());
    circuit_qoqo += PragmaActiveReset::new(1);
    circuit_qoqo += MeasureQubit::new(0, "c".into(), 0);

    assert_eq!(circuit_from_file, circuit_qoqo);
}

/// Test errors
#[test]
fn test_errors() {
    let file = File::open(
        std::env::current_dir()
            .unwrap()
            .join("tests/incorrect_file.qasm"),
    )
    .unwrap();
    let result = file_to_circuit(file);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("unknown parsing error"));
}

#[test]
fn test_single_qubit_gate() {
    let file = File::open(std::env::current_dir().unwrap().join("tests/sqg.qasm")).unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += SingleQubitGate::new(
        2,
        0.0.into(),
        0.0.into(),
        0.0.into(),
        (-1.0).into(),
        0.0.into(),
    );

    let sq: SingleQubitGateOperation = circuit_from_file.get(1).unwrap().try_into().unwrap();
    assert!(is_close(sq.alpha_r().float().unwrap().into(), 0.0.into()));
    assert!(is_close(sq.alpha_i().float().unwrap().into(), 0.0.into()));
    assert!(is_close(sq.beta_r().float().unwrap().into(), 0.0.into()));
    assert!(is_close(sq.beta_i().float().unwrap().into(), (-1.0).into()));
}

#[test]
fn test_comments() {
    let file = File::open(std::env::current_dir().unwrap().join("tests/comments.qasm")).unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 2, true);
    circuit_qoqo += RotateZ::new(0, 0.2.into());
    circuit_qoqo += RotateX::new(2, 2.1.into());
    circuit_qoqo += Hadamard::new(0);
    circuit_qoqo += PauliY::new(1);

    assert_eq!(circuit_from_file, circuit_qoqo);
}

#[test]
fn test_gate_definitions() {
    let file = File::open(
        std::env::current_dir()
            .unwrap()
            .join("tests/gate_defs.qasm"),
    )
    .unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += Hadamard::new(0);

    assert_eq!(circuit_from_file, circuit_qoqo);
}

#[test]
fn test_include_line_skip() {
    let file = File::open(std::env::current_dir().unwrap().join("tests/include.qasm")).unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 2, true);
    circuit_qoqo += PauliX::new(0);
    circuit_qoqo += Hadamard::new(1);
    circuit_qoqo += RotateX::new(2, 2.3.into());
    circuit_qoqo += CNOT::new(0, 1);
    circuit_qoqo += MeasureQubit::new(0, "c".into(), 0);

    assert_eq!(circuit_from_file, circuit_qoqo);
}

#[allow(clippy::approx_constant)]
#[test]
fn test_symbols() {
    let file = File::open(
        std::env::current_dir()
            .unwrap()
            .join("tests/symbols_math_expr.qasm"),
    )
    .unwrap();

    let circuit_from_file = file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 3, true);
    circuit_qoqo += RotateZ::new(0, 3.141592653589793.into());
    circuit_qoqo += RotateX::new(1, 1.5707963267948966.into());
    circuit_qoqo += RotateY::new(2, 0.7853981633974483.into());
    circuit_qoqo += PhaseShiftState1::new(2, 3.5.into());
    circuit_qoqo += PhaseShiftState1::new(1, (-1.0).into());
    circuit_qoqo += ControlledPhaseShift::new(0, 1, 2.5.into());
    circuit_qoqo += RotateZ::new(2, 1.7320508075688767.into());
    circuit_qoqo += RotateZ::new(1, 2.718281828459045.into());
    circuit_qoqo += RotateZ::new(0, 1.0.into());
    circuit_qoqo += RotateX::new(1, 4.0.into());

    assert_eq!(circuit_from_file, circuit_qoqo);
}
