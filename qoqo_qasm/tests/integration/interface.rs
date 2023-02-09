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
//! Testing the qoqo-qasm Interface

use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::CircuitWrapper;
use qoqo::QoqoBackendError;
use qoqo::QoqoError;

use qoqo_qasm::{qasm_call_circuit, qasm_call_operation};

use qoqo_calculator::CalculatorFloat;
use roqoqo::RoqoqoBackendError;
use std::f64::consts::PI;

use roqoqo::operations::*;
use roqoqo::Circuit;

use test_case::test_case;

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

/// Test qasm_call_circuit with correct Circuit
#[test]
fn test_qasm_call_circuit() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 1, true);
    circuit += PauliX::new(0);
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let qasm_circ: Vec<String> = vec![
            "creg ro[1];".to_string(),
            "x qr[0];".to_string(),
            "measure qr[0] -> ro[0];".to_string(),
        ];

        assert_eq!(qasm_call_circuit(circuitpy, "qr").unwrap(), qasm_circ)
    })
}

/// Test qasm_call_operation with correct Operations
/// Test that all operations return the correct String
#[test_case(Operation::from(PauliX::new(0)), "x q[0];"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "y q[0];"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)), "z q[0];"; "PauliZ")]
#[test_case(Operation::from(Hadamard::new(0)), "h q[0];"; "Hadamard")]
#[test_case(Operation::from(SGate::new(0)), "s q[0];"; "SGate")]
#[test_case(Operation::from(TGate::new(0)), "t q[0];"; "TGate")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(-PI))), "rx(-3.141592653589793) q[0];"; "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(-PI))), "ry(-3.141592653589793) q[0];"; "RotateY")]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(-PI))), "rz(-3.141592653589793) q[0];"; "RotateZ")]
#[test_case(Operation::from(SqrtPauliX::new(0)), "rx(pi/2) q[0];"; "SqrtPauliX")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), "rxx(pi/2) q[0],q[1];"; "MolmerSorensenXX")]
#[test_case(Operation::from(CNOT::new(0, 1)), "cx q[0],q[1];"; "CNOT")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "cy q[0],q[1];"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), "cz q[0],q[1];"; "ControlledPauliZ")]
// #[test_case(Operation::from(SingleQubitGate::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), "u3(0.000000000000000,0.000000000000000,0.000000000000000) q[0];"; "SingleQubitGate")]
#[test_case(Operation::from(PragmaRepeatedMeasurement::new("ro".to_string(), 1, None)), "measure q -> ro;"; "PragmaRepeatedMeasurement")]
#[test_case(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)), "measure q[0] -> ro[0];"; "MeasureQubit")]
#[test_case(Operation::from(DefinitionFloat::new("ro".to_string(), 1, true)), "creg ro[1];"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionUsize::new("ro".to_string(), 1, true)), "creg ro[1];"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new("ro".to_string(), 1, true)), "creg ro[1];"; "DefinitionBit")]
#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 1, true)), "creg ro[1];"; "DefinitionComplex")]
#[test_case(Operation::from(InputSymbolic::new("other".to_string(), 0.0)), ""; "InputSymbolic")]
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(20, "ro".to_string())), ""; "PragmaSetNumberOfMeasurements")]
fn test_qasm_call_operation(operation: Operation, converted: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_op: Py<PyAny> = convert_operation_to_pyobject(operation).unwrap();
        assert_eq!(
            qasm_call_operation(new_op.as_ref(py), "q").unwrap(),
            converted.to_string()
        )
    })
}

/// Test qasm_call_operation and qasm_call_circuit errors
#[test_case(Operation::from(ISwap::new(0, 1)))]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(0.23))))]
#[test_case(Operation::from(FSwap::new(0, 1)))]
#[test_case(Operation::from(RotateXY::new(
    0,
    CalculatorFloat::from(0.23),
    CalculatorFloat::from(0.23)
)))]
fn test_qasm_call_error(operation: Operation) {
    let mut wrong_circuit = Circuit::new();
    wrong_circuit += operation.clone();

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        let wrongcircuitpy = circuitpy_from_circuitru(py, wrong_circuit.clone());

        let result = qasm_call_circuit(dict.as_ref(), "qr");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Circuit: {:?}",
                QoqoBackendError::CannotExtractObject
            ))
            .to_string()
        );

        let result = qasm_call_circuit(wrongcircuitpy, "qr");
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

        let result = qasm_call_operation(dict.as_ref(), "qr");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Operation: {:?}",
                QoqoError::ConversionError
            ))
            .to_string()
        );
    })
}
