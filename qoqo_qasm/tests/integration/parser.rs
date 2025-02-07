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
//! Testing the qoqo-qasm parsing functions

use pyo3::{exceptions::PyFileNotFoundError, prelude::*};

use qoqo::{operations::convert_operation_to_pyobject, CircuitWrapper};
use roqoqo::{operations::*, Circuit};

use qoqo_qasm::qasm_file_to_circuit;

// helper functions
fn circuitpy_from_circuitru(py: Python, circuit: Circuit) -> Bound<CircuitWrapper> {
    let circuit_type = py.get_type_bound::<CircuitWrapper>();
    let binding = circuit_type.call0().unwrap();
    let circuitpy = binding.downcast::<CircuitWrapper>().unwrap();
    for op in circuit {
        let new_op = convert_operation_to_pyobject(op).unwrap();
        circuitpy.call_method1("add", (new_op.clone_ref(py),)).unwrap();
    }
    circuitpy.to_owned()
}

/// Test correct functionality with basic circuit
#[test]
fn test_qasm_file_to_circuit_correct() {
    let file = std::env::current_dir().unwrap().join("tests/input.qasm");

    let circuit = qasm_file_to_circuit(file.to_str().unwrap());

    assert!(circuit.is_ok());

    let mut circuit_qoqo = Circuit::new();
    circuit_qoqo += DefinitionBit::new("c".into(), 2, true);
    circuit_qoqo += PauliX::new(0);
    circuit_qoqo += Hadamard::new(1);
    circuit_qoqo += RotateX::new(2, 2.3.into());
    circuit_qoqo += CNOT::new(0, 1);
    circuit_qoqo += MeasureQubit::new(0, "c".into(), 0);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let circuitpy = circuitpy_from_circuitru(py, circuit_qoqo);
        let circuit_wrapper = circuitpy.extract::<CircuitWrapper>().unwrap();

        assert_eq!(circuit.unwrap(), circuit_wrapper);
    })
}

/// Test file error
#[test]
fn test_qasm_file_to_circuit_file_error() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let result = qasm_file_to_circuit("test");
        assert!(result.is_err());
        assert!(result
            .as_ref()
            .unwrap_err()
            .to_string()
            .contains("Error during File opening:"));
        assert!(result
            .unwrap_err()
            .is_instance_of::<PyFileNotFoundError>(py));
    })
}
