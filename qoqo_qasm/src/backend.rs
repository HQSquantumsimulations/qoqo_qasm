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

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};
use qoqo::convert_into_circuit;
use roqoqo_qasm::Backend;
use std::path::Path;

/// Backend to qoqo that produces QASM output which can be imported.
///
/// This backend takes a qoqo circuit to be run on a certain device and returns a QASM file
/// containing the translated circuit. The circuit itself is translated using the qoqo_qasm
/// interface. In this backend, the initialization sets up the relevant parameters and the run
/// function calls the QASM interface and writes the QASM file, which is saved to be used by the
/// user on whatever platform they see fit. QASM input is widely supported on various quantum
/// computing platforms.
#[pyclass(name = "QasmBackend", module = "qoqo_qasm")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QasmBackendWrapper {
    /// Internal storage of [roqoqo_qasm::Backend]
    pub internal: Backend,
}

#[pymethods]
impl QasmBackendWrapper {
    /// Creates new QASM backend.
    ///
    /// Args:
    ///     qubit_register_name (Optional[str]): The name of the qubit register.
    ///
    /// Returns:
    ///     Self: The new QasmBackend intance.
    #[new]
    pub fn new(
        qubit_register_name: Option<String>,
        qasm_version: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            internal: Backend::new(qubit_register_name, qasm_version)
                .map_err(|x| PyValueError::new_err(format!("{x}")))?,
        })
    }

    /// Translates a Circuit to a valid QASM string.
    ///
    /// Args:
    ///     circuit: The Circuit items that is translated
    ///
    /// Returns:
    ///     str: The valid QASM string
    ///
    /// Raises:
    ///     TypeError: Circuit conversion error
    ///     ValueError: Operation not in QASM backend
    #[pyo3(text_signature = "($self, circuit)")]
    pub fn circuit_to_qasm_str(&self, circuit: &PyAny) -> PyResult<String> {
        let circuit = convert_into_circuit(circuit).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Circuit: {x:?}"))
        })?;
        roqoqo_qasm::Backend::circuit_to_qasm_str(&self.internal, &circuit)
            .map_err(|x| PyValueError::new_err(format!("Error during QASM translation: {x:?}")))
    }

    /// Translates a Circuit to a QASM file.
    ///
    /// Args:
    ///     circuit: The Circuit that is translated
    ///     folder_name: The name of the folder that is prepended to all filenames.
    ///     filename: The name of the file the QASM text is saved to.
    ///     overwrite: Whether to overwrite file if it already exists.
    ///
    /// Returns:
    ///     Ok(()): The qasm file was correctly written
    ///
    /// Raises:
    ///     TypeError: Circuit conversion error
    ///     ValueError: Operation not in QASM backend
    #[pyo3(text_signature = "($self, circuit, folder_name, filename, overwrite)")]
    pub fn circuit_to_qasm_file(
        &self,
        circuit: &PyAny,
        folder_name: String,
        filename: String,
        overwrite: bool,
    ) -> PyResult<()> {
        let circuit = convert_into_circuit(circuit).map_err(|x| {
            PyTypeError::new_err(format!("Cannot convert python object to Circuit: {x:?}"))
        })?;
        let folder_name = Path::new(&folder_name);
        let filename = Path::new(&filename);
        roqoqo_qasm::Backend::circuit_to_qasm_file(
            &self.internal,
            &circuit,
            folder_name,
            filename,
            overwrite,
        )
        .map_err(|x| PyValueError::new_err(format!("Error during QASM translation: {x:?}")))
    }
}
