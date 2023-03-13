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

use qoqo_qasm::qasm_gate_definition;
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
        .downcast::<PyCell<CircuitWrapper>>()
        .unwrap();
    for op in circuit {
        let new_op = convert_operation_to_pyobject(op).unwrap();
        circuitpy.call_method1("add", (new_op.clone(),)).unwrap();
    }
    circuitpy
}

/// Test qasm_call_circuit with correct Circuit
#[test_case("2.0", "qreg qr[2]", "creg ro[1]"; "2.0")]
#[test_case("3.0", "qubit[2] qr", "bit[1] ro"; "3.0")]
fn test_qasm_call_circuit(qasm_version: &str, _qubits: &str, bits: &str) {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 1, false);
    circuit += PauliX::new(0);
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let qasm_circ: Vec<String> = vec![
            format!("{bits};"),
            "x qr[0];".to_string(),
            "measure qr[0] -> ro[0];".to_string(),
        ];

        assert_eq!(
            qasm_call_circuit(circuitpy, "qr", qasm_version).unwrap(),
            qasm_circ
        )
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
#[test_case(Operation::from(SqrtPauliX::new(0)), "sx q[0];"; "SqrtPauliX")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), "rxx(pi/2) q[0],q[1];"; "MolmerSorensenXX")]
#[test_case(Operation::from(CNOT::new(0, 1)), "cx q[0],q[1];"; "CNOT")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "cy q[0],q[1];"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), "cz q[0],q[1];"; "ControlledPauliZ")]
// #[test_case(Operation::from(SingleQubitGate::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), "u3(0.000000000000000,0.000000000000000,0.000000000000000) q[0];"; "SingleQubitGate")]
#[test_case(Operation::from(PragmaRepeatedMeasurement::new("ro".to_string(), 1, None)), "measure q -> ro;"; "PragmaRepeatedMeasurement")]
#[test_case(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)), "measure q[0] -> ro[0];"; "MeasureQubit")]
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(20, "ro".to_string())), ""; "PragmaSetNumberOfMeasurements")]
fn test_qasm_call_operation_identical_2_3(operation: Operation, converted: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_op: Py<PyAny> = convert_operation_to_pyobject(operation).unwrap();
        assert_eq!(
            qasm_call_operation(new_op.as_ref(py), "q", "2.0").unwrap(),
            converted.to_string()
        );
        assert_eq!(
            qasm_call_operation(new_op.as_ref(py), "q", "3.0").unwrap(),
            converted.to_string()
        );
    })
}

#[test_case(Operation::from(DefinitionFloat::new("ro".to_string(), 1, true)), "creg ro[1];", "output float[1] ro;"; "DefinitionFloat output")]
#[test_case(Operation::from(DefinitionFloat::new("ro".to_string(), 1, false)), "creg ro[1];", "float[1] ro;"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionUsize::new("ro".to_string(), 1, true)), "creg ro[1];", "output uint[1] ro;"; "DefinitionUsize ouput")]
#[test_case(Operation::from(DefinitionUsize::new("ro".to_string(), 1, false)), "creg ro[1];", "uint[1] ro;"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new("ro".to_string(), 1, true)), "creg ro[1];", "output bit[1] ro;"; "DefinitionBit output")]
#[test_case(Operation::from(DefinitionBit::new("ro".to_string(), 1, false)), "creg ro[1];", "bit[1] ro;"; "DefinitionBit")]
#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 1, true)), "creg ro[1];", "output float[1] ro_re;\noutput float[1] ro_im;"; "DefinitionComplex output")]
#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 1, false)), "creg ro[1];", "float[1] ro_re;\nfloat[1] ro_im;"; "DefinitionComplex")]
#[test_case(Operation::from(InputSymbolic::new("other".to_string(), 0.0)), "", "input float other;"; "InputSymbolic")]
#[test_case(Operation::from(InputBit::new("other".to_string(), 0, false)), "", "other[0] = false;"; "InputBit")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(1.0))), "", "gphase 1e0;"; "PragmaGlobalPhase")]
fn test_qasm_call_operation_different_2_3(
    operation: Operation,
    converted_2: &str,
    converted_3: &str,
) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_op: Py<PyAny> = convert_operation_to_pyobject(operation).unwrap();
        assert_eq!(
            qasm_call_operation(new_op.as_ref(py), "q", "2.0").unwrap(),
            converted_2.to_string()
        );
        assert_eq!(
            qasm_call_operation(new_op.as_ref(py), "q", "3.0").unwrap(),
            converted_3.to_string()
        );
    })
}

/// Test qasm_call_operation, qasm_call_circuit and qasm_gate_definition errors
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
fn test_qasm_call_error(operation: Operation, qasm_version: &str) {
    let mut wrong_circuit = Circuit::new();
    wrong_circuit += operation.clone();

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let dict = PyDict::new(py);
        let wrongcircuitpy = circuitpy_from_circuitru(py, wrong_circuit.clone());
        let wrongoperationpy = convert_operation_to_pyobject(operation.clone()).unwrap();

        let result = qasm_call_circuit(dict.as_ref(), "qr", qasm_version);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Circuit: {:?}",
                QoqoBackendError::CannotExtractObject
            ))
            .to_string()
        );

        let result = qasm_call_circuit(wrongcircuitpy, "qr", qasm_version);
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

        let result = qasm_call_operation(dict.as_ref(), "qr", qasm_version);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Operation: {:?}",
                QoqoError::ConversionError
            ))
            .to_string()
        );

        let result = qasm_call_operation(wrongoperationpy.as_ref(py), "qr", qasm_version);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyValueError::new_err(format!(
                "Error during QASM translation: {:?}",
                RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang()
                }
            ))
            .to_string()
        );

        let result = qasm_gate_definition(dict.as_ref());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Operation: {:?}",
                QoqoError::ConversionError
            ))
            .to_string()
        );

        let result = qasm_gate_definition(wrongoperationpy.as_ref(py));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyValueError::new_err(format!(
                "Error during QASM gate definition: {:?}",
                RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang()
                }
            ))
            .to_string()
        );
    })
}

/// Test qasm_gate_definition call
#[test_case(Operation::from(PauliX::new(0)), "gate x a { u3(pi,0,pi) a; }"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "gate y a { u3(pi,pi/2,pi/2) a; }"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)), "gate z a { u1(pi) a; }"; "PauliZ")]
fn test_qasm_gate_definition(operation: Operation, converted: &str) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let new_op: Py<PyAny> = convert_operation_to_pyobject(operation).unwrap();
        assert_eq!(
            qasm_gate_definition(new_op.as_ref(py)).unwrap(),
            converted.to_string()
        )
    })
}
