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

use pyo3::{prelude::*, exceptions::PyTypeError};
use qoqo::QoqoError;
use qoqo::operations::convert_pyany_to_operation;
use roqoqo::Circuit;
use roqoqo_qasm::{call_circuit, call_operation};

/// Translate the qoqo circuit into QASM ouput
///
/// The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
/// to QASM output (strings).
///
/// Args:
///     circuit: The qoqo circuit that is translated
///     number_qubits: The number of qubits in the circuit
///     qubit_names: The dictionary of qubit names to translate the circuit to
///     use_symbolic: Whether to use symbolic translation (True) or not (False)
///
/// Returns:
///     Tuple[List[str], Dict[int, str]]: translated circuit
#[pyfunction]
pub fn qasm_call_circuit(circuit: Py<PyAny>) -> PyResult<String>{
    let circuit = Python::with_gil(|py| -> Result<Circuit, QoqoError> {
        let circ_ref = circuit.as_ref(py);
        qoqo::convert_into_circuit(circ_ref)
    })?;

}

/// Translate a qoqo operation to QASM text
///
/// Args:
///     operation: The qoqo operation that is translated
///     number_qubits: The number of qubits in the circuit
///     qubit_names: The dictionary of qubit names to translate the operation to
///
/// Returns:
///     str: translated operation
///
/// Raises:
///     RuntimeError: Operation not in QASM backend
#[pyfunction]
pub fn qasm_call_operation(operation: &PyAny) -> Result<String, RoqoqoBackendError>{
    let operation = convert_pyany_to_operation(op).map_err(|x| {
        PyTypeError::new_err(format!("Cannot convert python object to Operation {:?}", x))
    })?;
}