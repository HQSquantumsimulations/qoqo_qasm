// Copyright © 2022 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//! Testing the qoqo-qasm Backend

/// Test simple circuit with a Definition, a GateOperation and a PragmaOperation
use pyo3::prelude::*;

use std::env::temp_dir;
use std::fs;
use std::path::Path;

use qoqo_qasm::QasmBackendWrapper;

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::CircuitWrapper;

use roqoqo::operations::*;
use roqoqo::Circuit;

// helper functions
fn circuitpy_from_circuitru(py: Python, circuit: Circuit) -> &PyCell<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    let circuitpy = circuit_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CircuitWrapper>>()
        .unwrap();
    for op in circuit {
        let new_op = convert_operation_to_pyobject(op).unwrap();
        circuitpy.call_method1("add", (new_op.clone(),)).unwrap();
    }
    circuitpy
}

fn new_qasmbackend(py: Python, qubit_register_name: Option<String>) -> &PyCell<QasmBackendWrapper> {
    let circuit_type = py.get_type::<QasmBackendWrapper>();
    circuit_type
        .call1((qubit_register_name,))
        .unwrap()
        .cast_as::<PyCell<QasmBackendWrapper>>()
        .unwrap()
}

/// Test run_circuit_to_qasm_str on a simple Circuit
#[test]
fn run_circuit_to_str() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qasmbackend(py, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        backendpy
            .call_method1("circuit_to_qasm_str", (circuitpy,))
            .unwrap();
    })
}

/// Test run_circuit_to_qasm_file on a simple Circuit
#[test]
fn run_circuit_to_file() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qasmbackend(py, Some("qr".to_string()));
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        backendpy
            .call_method1(
                "circuit_to_qasm_file",
                (circuitpy, temp_dir().to_str().unwrap(), "fnametest", true),
            )
            .unwrap();

        let lines = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\nqreg qr[2];\ncreg ro[2];\nrx(1.5707963267948966) qr[0];\nx qr[1];\nmeasure qr -> ro;\n");
        let read_in_path = temp_dir().join(Path::new("fnametest.qasm"));
        let extracted = fs::read_to_string(&read_in_path);
        fs::remove_file(&read_in_path).unwrap();
        assert_eq!(lines, extracted.unwrap());
    })
}

/// Test that backend returns error when running for a file that exists without overwrite
#[test]
fn run_error() {}

/// Test Debug, Clone and PartialEq for Backend
#[test]
fn test_debug_clone_partialeq() {}
