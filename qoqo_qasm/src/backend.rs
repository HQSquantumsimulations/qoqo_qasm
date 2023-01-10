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

use pyo3::prelude::*;
use roqoqo_qasm::Backend;

/// Backend to qoqo that produces QASM output which can be imported.
///
/// This backend takes a qoqo circuit to be run on a certain device and returns a QASM file
/// containing the translated circuit. The circuit itself is translated using the qoqo_qasm
/// interface. In this backend, the initialization sets up the relevant parameters and the run
/// function calls the QASM interface and writes the QASM file, which is saved to be used by the
/// user on whatever platform they see fit. QASM input is widely supported on various quantum
/// computing platforms.
#[pyclass(name = "QasmBackend")]
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
    pub fn new(qubit_register_name: Option<String>) -> Self {
        Self {
            internal: Backend::new(qubit_register_name),
        }
    }
}
