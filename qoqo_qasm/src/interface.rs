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

use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::{exceptions::PyTypeError, prelude::*};
use qoqo::convert_into_circuit;
use qoqo::operations::convert_pyany_to_operation;
use roqoqo_qasm::{call_circuit, call_operation, gate_definition, QasmVersion};

/// Translate the qoqo circuit into QASM ouput
///
/// The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
/// to QASM output (strings).
///
/// Args:
///     circuit (Circuit): The qoqo circuit that is translated
///     qubit_register_name (str): The name of the quantum register
///
/// Returns:
///     List[str]: The translated circuit
///
/// Raises:
///     TypeError: Circuit conversion error
///     ValueError: Operation not in QASM backend
#[pyfunction]
pub fn qasm_call_circuit(
    circuit: &PyAny,
    qubit_register_name: &str,
    qasm_version: &str,
) -> PyResult<Vec<String>> {
    let circuit = convert_into_circuit(circuit).map_err(|x| {
        PyTypeError::new_err(format!("Cannot convert python object to Circuit: {x:?}"))
    })?;
    call_circuit(
        &circuit,
        qubit_register_name,
        QasmVersion::from_str(qasm_version).map_err(|x| PyValueError::new_err(format!("{x}")))?,
    )
    .map_err(|x| PyValueError::new_err(format!("Error during QASM translation: {x:?}")))
}

/// Translate a qoqo operation to QASM text
///
/// Args:
///     operation: The qoqo operation that is translated
///     qubit_register_name (str): The name of the quantum register
///
/// Returns:
///     str: The translated operation
///
/// Raises:
///     TypeError: Operation conversion error
///     ValueError: Operation not in QASM backend
#[pyfunction]
pub fn qasm_call_operation(
    operation: &PyAny,
    qubit_register_name: &str,
    qasm_version: &str,
) -> PyResult<String> {
    let operation = convert_pyany_to_operation(operation).map_err(|x| {
        PyTypeError::new_err(format!("Cannot convert python object to Operation: {x:?}"))
    })?;
    call_operation(
        &operation,
        qubit_register_name,
        QasmVersion::from_str(qasm_version).map_err(|x| PyValueError::new_err(format!("{x}")))?,
    )
    .map_err(|x| PyValueError::new_err(format!("Error during QASM translation: {x:?}")))
}

/// Outputs the QASM gate definition of many qoqo operations
///
/// Args:
///     operation: The qoqo Operation to be defined
///
/// Returns:
///     str: The gate QASM gate definition.
///
/// Raises:
///     ValueError: Operation-specific error or Operation not in QASM backend
#[pyfunction]
pub fn qasm_gate_definition(operation: &PyAny, qasm_version: &str) -> PyResult<String> {
    let operation = convert_pyany_to_operation(operation).map_err(|x| {
        PyTypeError::new_err(format!("Cannot convert python object to Operation: {x:?}"))
    })?;
    let qasm_version =
        QasmVersion::from_str(qasm_version).map_err(|x| PyValueError::new_err(format!("{x}")))?;
    gate_definition(&operation, qasm_version)
        .map_err(|x| PyValueError::new_err(format!("Error during QASM gate definition: {x:?}")))
}
