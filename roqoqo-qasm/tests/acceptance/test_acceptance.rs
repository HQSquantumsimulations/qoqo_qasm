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
//! Test roqoqo QASM acceptance

use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit};
use roqoqo_qasm::Backend;
use std::env::temp_dir;
use std::fs;
use std::path::Path;

/// Test generating a file for gate operations with QASM interface
#[test]
fn test_acceptance_with_qasmbackend() {
    let backend = Backend::new(Some("qr".to_string()), None);

    let mut circuit = Circuit::new();
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += RotateY::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += RotateZ::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += CNOT::new(0, 1);
    circuit += Hadamard::new(0);
    circuit += PauliX::new(0);
    circuit += PauliY::new(0);
    circuit += PauliZ::new(0);
    circuit += SGate::new(0);
    circuit += TGate::new(0);
    circuit += SqrtPauliX::new(0);
    circuit += MolmerSorensenXX::new(0, 1);
    circuit += ControlledPauliY::new(0, 1);
    circuit += ControlledPauliZ::new(0, 1);
    circuit += SingleQubitGate::new(
        0,
        CalculatorFloat::from(1),
        CalculatorFloat::from(0),
        CalculatorFloat::from(1),
        CalculatorFloat::from(0),
        CalculatorFloat::from(1.0),
    );
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 1, None);
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);
    circuit += DefinitionFloat::new("rof".to_string(), 1, true);
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += DefinitionComplex::new("roc".to_string(), 1, true);
    circuit += InputSymbolic::new("other".to_string(), 0.0);
    circuit += PragmaSetNumberOfMeasurements::new(20, "ro".to_string());
    backend
        .circuit_to_qasm_file(
            &circuit,
            temp_dir().as_path(),
            Path::new("test_simple_rust.qasm"),
            true,
        )
        .unwrap();

    let read_in_path = temp_dir().join(Path::new("test_simple_rust.qasm"));
    let b = read_in_path.exists();
    fs::remove_file(&read_in_path).unwrap();
    assert!(b);
}
