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
//! Testing the roqoqo-qasm Backend

use roqoqo::prelude::*;
use roqoqo::{operations::*, Circuit};
use roqoqo_qasm::Backend;
// use roqoqo_test::prepare_monte_carlo_gate_test;
use std::env::temp_dir;
use std::fs;
use std::path::Path;
/// Test simple circuit with a Definition, a GateOperation and a PragmaOperation
#[test]
fn run_simple_circuit() {
    let backend = Backend::new(Some("qr".to_string()));
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), None, 20);
    let _ = backend
        .circuit_to_qasm_file(
            &circuit,
            temp_dir().as_path(),
            &Path::new("test_simple0"),
            true,
        )
        .unwrap();

    let lines = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\nqreg qr[2];\ncreg ro[2];\nrx(1.5707963267948966) qr[0];\nx qr[1];\nmeasure qr -> ro;\n");
    let read_in_path = temp_dir().join(Path::new("test_simple0.qasm"));
    let extracted = fs::read_to_string(&read_in_path);
    fs::remove_file(&read_in_path).unwrap();
    assert_eq!(lines, extracted.unwrap());
}

/// Test simple circuit with a Definition, a GateOperation and a PragmaOperation
#[test]
fn simple_circuit_iterator_to_file() {
    let backend = Backend::new(None);
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), None, 20);
    let _ = backend
        .circuit_iterator_to_qasm_file(
            circuit.iter(),
            temp_dir().as_path(),
            Path::new("test_simple1"),
            true,
        )
        .unwrap();

    let lines = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\nqreg q[2];\ncreg ro[2];\nrx(1.5707963267948966) q[0];\nx q[1];\nmeasure q -> ro;\n");
    let read_in_path = temp_dir().join(Path::new("test_simple1.qasm"));
    let extracted = fs::read_to_string(&read_in_path);
    fs::remove_file(&read_in_path).unwrap();
    assert_eq!(lines, extracted.unwrap());
}

/// Test that backend returns error when running for a file that exists without overwrite
#[test]
fn run_error() {
    let backend = Backend::new(None);
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    let _ = backend.circuit_to_qasm_file(
        &circuit,
        temp_dir().as_path(),
        Path::new("test_simple"),
        false,
    );
    let error = backend.circuit_to_qasm_file(
        &circuit,
        temp_dir().as_path(),
        Path::new("test_simple"),
        false,
    );
    let read_in_path = temp_dir().join(Path::new("test_simple.qasm"));
    assert_eq!(
        error,
        Err(RoqoqoBackendError::FileAlreadyExists {
            path: read_in_path.to_str().unwrap().to_string()
        })
    );
}

/// Test Debug, Clone and PartialEq for Backend
#[test]
fn test_debug_clone_partialeq() {
    let backend = Backend::new(Some("qtest".to_string()));

    // Test Debug trait
    assert_eq!(
        format!("{:?}", backend),
        "Backend { qubit_register_name: \"qtest\" }"
    );

    // Test Clone trait
    assert_eq!(backend.clone(), backend);

    // PartialEq
    let backend_0 = Backend::new(Some("qtest".to_string()));
    let backend_2 = Backend::new(Some("q".to_string()));

    assert!(backend_0 == backend);
    assert!(backend == backend_0);
    assert!(backend_2 != backend);
    assert!(backend != backend_2);
}
