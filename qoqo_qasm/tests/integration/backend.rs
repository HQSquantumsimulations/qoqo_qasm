// Copyright © 2022-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qoqo::QoqoBackendError;
use roqoqo::RoqoqoBackendError;

use std::env::temp_dir;
use std::fs;
use std::path::Path;

use qoqo_calculator::CalculatorFloat;

use qoqo_qasm::QasmBackendWrapper;

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::CircuitWrapper;

use roqoqo::operations::*;
use roqoqo::Circuit;

use test_case::test_case;

// helper functions
fn circuitpy_from_circuitru(py: Python, circuit: Circuit) -> &PyCell<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    let circuitpy = circuit_type
        .call0()
        .unwrap()
        .downcast::<PyCell<CircuitWrapper>>()
        .unwrap();
    for op in circuit {
        let new_op = convert_operation_to_pyobject(op).unwrap();
        circuitpy.call_method1("add", (new_op.clone(),)).unwrap();
    }
    circuitpy
}

fn new_qasmbackend(
    py: Python,
    qubit_register_name: Option<String>,
    qasm_version: Option<String>,
) -> &PyCell<QasmBackendWrapper> {
    let backend_type = py.get_type::<QasmBackendWrapper>();
    backend_type
        .call1((qubit_register_name, qasm_version))
        .unwrap()
        .downcast::<PyCell<QasmBackendWrapper>>()
        .unwrap()
}

/// Test circuit_to_qasm_str on a simple Circuit
#[test_case("2.0", "qreg q[2]", "creg ro[2]"; "2.0")]
#[test_case("3.0", "qubit[2] q", "bit[2] ro"; "3.0")]
fn test_circuit_to_qasm_str(qasm_version: &str, qubits: &str, bits: &str) {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, false);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qasmbackend(py, None, Some(qasm_version.to_string()));
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let result: String = backendpy
            .call_method1("circuit_to_qasm_str", (circuitpy,))
            .unwrap()
            .extract()
            .unwrap();
        let cnot = if qasm_version == "2.0" {
            "CX"
        } else {
            "ctrl @ x"
        };
        let lines = format!("OPENQASM {qasm_version};\n\ngate u3(theta,phi,lambda) q {{ U(theta,phi,lambda) q; }}\ngate u2(phi,lambda) q {{ U(pi/2,phi,lambda) q; }}\ngate u1(lambda) q {{ U(0,0,lambda) q; }}\ngate rx(theta) a {{ u3(theta,-pi/2,pi/2) a; }}\ngate ry(theta) a {{ u3(theta,0,0) a; }}\ngate rz(phi) a {{ u1(phi) a; }}\ngate cx c,t {{ {cnot} c,t; }}\n\ngate x a {{ u3(pi,0,pi) a; }}\n\n{qubits};\n\n{bits};\nrx(1.5707963267948966e0) q[0];\nx q[1];\nmeasure q -> ro;\n");
        assert_eq!(lines, result);
    })
}

/// Test circuit_to_qasm_file on a simple Circuit
#[test_case("2.0", "qreg qr[2]", "creg ro[2]"; "2.0")]
#[test_case("3.0", "qubit[2] qr", "bit[2] ro"; "3.0")]
fn test_circuit_to_qasm_file(qasm_version: &str, qubits: &str, bits: &str) {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, false);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qasmbackend(py, Some("qr".to_string()), Some(qasm_version.to_string()));
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        backendpy
            .call_method1(
                "circuit_to_qasm_file",
                (circuitpy, temp_dir().to_str().unwrap(), "fnametest", true),
            )
            .unwrap();

        let cnot = if qasm_version == "2.0" {
            "CX"
        } else {
            "ctrl @ x"
        };
        let lines = format!("OPENQASM {qasm_version};\n\ngate u3(theta,phi,lambda) q {{ U(theta,phi,lambda) q; }}\ngate u2(phi,lambda) q {{ U(pi/2,phi,lambda) q; }}\ngate u1(lambda) q {{ U(0,0,lambda) q; }}\ngate rx(theta) a {{ u3(theta,-pi/2,pi/2) a; }}\ngate ry(theta) a {{ u3(theta,0,0) a; }}\ngate rz(phi) a {{ u1(phi) a; }}\ngate cx c,t {{ {cnot} c,t; }}\n\ngate x a {{ u3(pi,0,pi) a; }}\n\n{qubits};\n\n{bits};\nrx(1.5707963267948966e0) qr[0];\nx qr[1];\nmeasure qr -> ro;\n");
        let read_in_path = temp_dir().join(Path::new("fnametest.qasm"));
        let extracted = fs::read_to_string(&read_in_path);
        fs::remove_file(&read_in_path).unwrap();
        assert_eq!(lines, extracted.unwrap());
    })
}

/// Test circuit_to_qasm_str and circuit_to_qasm_file errors
#[test_case(Operation::from(Bogoliubov::new(
    0,
    1,
    CalculatorFloat::from(0.2),
    CalculatorFloat::from(0.3)
)), "2.0"; "bog, 2.0")]
#[test_case(Operation::from(Bogoliubov::new(
    0,
    1,
    CalculatorFloat::from(0.2),
    CalculatorFloat::from(0.3)
)), "3.0"; "bog, 3.0")]
#[test_case(Operation::from(ComplexPMInteraction::new(
    0,
    1,
    CalculatorFloat::from(0.3),
    CalculatorFloat::from(0.2)
)), "2.0"; "complexpm, 2.0")]
#[test_case(Operation::from(ComplexPMInteraction::new(
    0,
    1,
    CalculatorFloat::from(0.3),
    CalculatorFloat::from(0.2)
)), "3.0"; "complexpm, 3.0")]
#[test_case(Operation::from(GPi::new(
    0,
    CalculatorFloat::from(0.3),
)), "2.0"; "gpi, 2.0")]
#[test_case(Operation::from(GPi::new(
    0,
    CalculatorFloat::from(0.3),
)), "3.0"; "gpi, 3.0")]
#[test_case(Operation::from(GPi2::new(
    0,
    CalculatorFloat::from(0.3),
)), "2.0"; "gpi2, 2.0")]
#[test_case(Operation::from(GPi2::new(
    0,
    CalculatorFloat::from(0.3),
)), "3.0"; "gpi2, 3.0")]
fn test_circuit_to_qasm_error(operation: Operation, qasm_version: &str) {
    let mut wrong_circuit = Circuit::new();
    wrong_circuit += operation.clone();

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let wrongcircuitpy = circuitpy_from_circuitru(py, wrong_circuit.clone());

        let backendpy = new_qasmbackend(py, None, Some(qasm_version.to_string()));
        let result = backendpy.call_method1("circuit_to_qasm_str", (3,));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Circuit: {:?}",
                QoqoBackendError::CannotExtractObject
            ))
            .to_string()
        );

        let result = backendpy.call_method1("circuit_to_qasm_str", (wrongcircuitpy,));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyValueError::new_err(format!(
                "Error during QASM translation: {:?}",
                RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang(),
                }
            ))
            .to_string()
        );

        let backendpy = new_qasmbackend(py, None, Some(qasm_version.to_string()));
        let result = backendpy.call_method1(
            "circuit_to_qasm_file",
            (3, temp_dir().to_str().unwrap(), "fnametest", true),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Circuit: {:?}",
                QoqoBackendError::CannotExtractObject
            ))
            .to_string()
        );

        let result = backendpy.call_method1(
            "circuit_to_qasm_file",
            (
                wrongcircuitpy,
                temp_dir().to_str().unwrap(),
                "fnametest",
                true,
            ),
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            PyValueError::new_err(format!(
                "Error during QASM translation: {:?}",
                RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang(),
                }
            ))
            .to_string()
        );
    })
}

#[test]
fn test_parsing_methods() {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let path = std::env::current_dir().unwrap().join("tests/input.qasm");
    let file = File::open(path.clone()).unwrap();
    let unparsed_file = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap() + "\n")
        .collect::<String>();

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backend = new_qasmbackend(py, None, None);
        let result = backend.call_method1("qasm_file_to_circuit", (path.to_str().unwrap(),));
        assert!(result.is_ok());

        let result = backend.call_method1("qasm_str_to_circuit", (unparsed_file,));
        assert!(result.is_ok());
    })
}
