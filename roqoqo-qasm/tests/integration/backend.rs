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
//! Testing the roqoqo-qasm Backend

use roqoqo::prelude::*;
use roqoqo::{operations::*, Circuit};
use roqoqo_qasm::Backend;
// use roqoqo_test::prepare_monte_carlo_gate_test;
use std::env::temp_dir;
use std::fs;
use std::path::Path;
use test_case::test_case;

/// Test simple circuit with a Definition, a GateOperation and a PragmaOperation
#[test_case("2.0", "qreg qr[2]", "creg ro[2]"; "2.0")]
#[test_case("3.0", "qubit[2] qr", "bit[2] ro"; "3.0")]
fn run_simple_circuit(qasm_version: &str, qubits: &str, bits: &str) {
    let backend = Backend::new(Some("qr".to_string()), Some(qasm_version.to_string())).unwrap();
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, false);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);
    let mut file_name = format!("test_simple0_{}", qasm_version.chars().next().unwrap());
    backend
        .circuit_to_qasm_file(
            &circuit,
            temp_dir().as_path(),
            Path::new(file_name.as_str()),
            true,
        )
        .unwrap();

    let cnot = if qasm_version == "2.0" {
        "CX"
    } else {
        "ctrl @ x"
    };
    let lines = format!("OPENQASM {qasm_version};\n\ngate u3(theta,phi,lambda) q {{ U(theta,phi,lambda) q; }}\ngate u2(phi,lambda) q {{ U(pi/2,phi,lambda) q; }}\ngate u1(lambda) q {{ U(0,0,lambda) q; }}\ngate rx(theta) a {{ u3(theta,-pi/2,pi/2) a; }}\ngate ry(theta) a {{ u3(theta,0,0) a; }}\ngate rz(phi) a {{ u1(phi) a; }}\ngate cx c,t {{ {cnot} c,t; }}\n\ngate x a {{ u3(pi,0,pi) a; }}\n\n{qubits};\n\n{bits};\nrx(1.5707963267948966) qr[0];\nx qr[1];\nmeasure qr -> ro;\n");
    file_name.push_str(".qasm");
    let read_in_path = temp_dir().join(Path::new(file_name.as_str()));
    let extracted = fs::read_to_string(&read_in_path);
    fs::remove_file(&read_in_path).unwrap();
    assert_eq!(lines, extracted.unwrap());
}

/// Test simple circuit with a Definition, a GateOperation and a PragmaOperation
#[test_case("2.0", "qreg q[2]", "creg ro[2]"; "2.0")]
#[test_case("3.0", "qubit[2] q", "bit[2] ro"; "3.0")]
fn simple_circuit_iterator_to_file(qasm_version: &str, qubits: &str, bits: &str) {
    let backend = Backend::new(None, Some(qasm_version.to_string())).unwrap();
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, false);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    let mut file_name = format!("test_simple1_{}", qasm_version.chars().next().unwrap());
    backend
        .circuit_iterator_to_qasm_file(
            circuit.iter(),
            temp_dir().as_path(),
            Path::new(file_name.as_str()),
            true,
        )
        .unwrap();
    let cnot = if qasm_version == "2.0" {
        "CX"
    } else {
        "ctrl @ x"
    };
    let lines = format!("OPENQASM {qasm_version};\n\ngate u3(theta,phi,lambda) q {{ U(theta,phi,lambda) q; }}\ngate u2(phi,lambda) q {{ U(pi/2,phi,lambda) q; }}\ngate u1(lambda) q {{ U(0,0,lambda) q; }}\ngate rx(theta) a {{ u3(theta,-pi/2,pi/2) a; }}\ngate ry(theta) a {{ u3(theta,0,0) a; }}\ngate rz(phi) a {{ u1(phi) a; }}\ngate cx c,t {{ {cnot} c,t; }}\n\ngate x a {{ u3(pi,0,pi) a; }}\n\n{qubits};\n\n{bits};\nrx(1.5707963267948966) q[0];\nx q[1];\nmeasure q -> ro;\n");
    file_name.push_str(".qasm");
    let read_in_path = temp_dir().join(Path::new(file_name.as_str()));
    let extracted = fs::read_to_string(&read_in_path);
    fs::remove_file(&read_in_path).unwrap();
    assert_eq!(lines, extracted.unwrap());
}

/// Test duplicate gates definitions
#[test_case("2.0", "qreg qr[2]", "creg ro[2]"; "2.0")]
#[test_case("3.0", "qubit[2] qr", "bit[2] ro"; "3.0")]
fn test_duplicate_definitions(qasm_version: &str, qubits: &str, bits: &str) {
    let backend = Backend::new(Some("qr".to_string()), Some(qasm_version.to_string())).unwrap();
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, false);
    circuit += PauliX::new(0);
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    let output = backend.circuit_to_qasm_str(&circuit).unwrap();
    let cnot = if qasm_version == "2.0" {
        "CX"
    } else {
        "ctrl @ x"
    };
    let lines = format!("OPENQASM {qasm_version};\n\ngate u3(theta,phi,lambda) q {{ U(theta,phi,lambda) q; }}\ngate u2(phi,lambda) q {{ U(pi/2,phi,lambda) q; }}\ngate u1(lambda) q {{ U(0,0,lambda) q; }}\ngate rx(theta) a {{ u3(theta,-pi/2,pi/2) a; }}\ngate ry(theta) a {{ u3(theta,0,0) a; }}\ngate rz(phi) a {{ u1(phi) a; }}\ngate cx c,t {{ {cnot} c,t; }}\n\ngate x a {{ u3(pi,0,pi) a; }}\n\n{qubits};\n\n{bits};\nx qr[0];\nx qr[1];\nmeasure qr -> ro;\n");
    assert_eq!(output, lines);
}

/// Test that backend returns error when running for a file that exists without overwrite
#[test]
fn run_error() {
    let backend = Backend::new(None, None).unwrap();
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, false);
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
    let backend = Backend::new(Some("qtest".to_string()), None).unwrap();

    // Test Debug trait
    assert_eq!(
        format!("{backend:?}"),
        "Backend { qubit_register_name: \"qtest\", qasm_version: V2point0 }"
    );

    // Test Clone trait
    assert_eq!(backend.clone(), backend);

    // PartialEq
    let backend_0 = Backend::new(Some("qtest".to_string()), None).unwrap();
    let backend_2 = Backend::new(Some("q".to_string()), None).unwrap();

    assert!(backend_0 == backend);
    assert!(backend == backend_0);
    assert!(backend_2 != backend);
    assert!(backend != backend_2);
}
