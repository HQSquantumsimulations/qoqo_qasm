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

use pyo3::prelude::*;
use pyo3::{exceptions::PyValueError, PyResult};
use qoqo::CircuitWrapper;
use std::fs::File;

use roqoqo_qasm::file_to_circuit;

/// Translates a QASM File to a Circuit.
///
/// Args:
///     file (str): The path to the QASM file.
///
/// Returns:
///     Circuit: The Circuit that was read from the QASM file.
///
/// Raises:
///     PyValueError: An error occurred while reading the QASM file or while converting the file into a Circuit.
#[pyfunction]
pub fn qasm_file_to_circuit(file: &str) -> PyResult<CircuitWrapper> {
    let f = File::open(file).map_err(|x| PyValueError::new_err(format!("Error during File opening: {x}")))?;

    let circuit = file_to_circuit(f).map_err(|x| PyValueError::new_err(format!("{x}")))?;

    Ok(CircuitWrapper { internal: circuit })
}
