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

use std::fs::File;

use roqoqo::Circuit;
use roqoqo::operations::*;

use roqoqo_qasm::qasm_file_to_circuit;

/// Test basic file
#[test]
fn test_basic_file() {
    let file = File::open(std::env::current_dir().unwrap().join("tests/input.qasm")).unwrap();

    let circuit_from_file = qasm_file_to_circuit(file).unwrap();

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 2, true);
    circuit_qoqo += PauliX::new(0);
    circuit_qoqo += Hadamard::new(1);
    circuit_qoqo += RotateX::new(2, 2.3.into());
    circuit_qoqo += CNOT::new(0, 1);
    circuit_qoqo += MeasureQubit::new(0, "c".into(), 0);

    assert_eq!(circuit_from_file, circuit_qoqo);
}
