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

use crate::call_operation;
use roqoqo::operations::*;
use roqoqo::{Circuit, RoqoqoBackendError};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::path::PathBuf;
use std::usize;
/// QASM backend to qoqo
///
/// This backend to roqoqo produces QASM output which can be exported.
///
/// This backend takes a roqoqo circuit and returns a QASM String or writes a QASM file
/// containing the translated circuit. The circuit itself is translated using the roqoqo-qasm
/// interface. In this backend, the initialization sets up the relevant parameters and the run
/// function calls the QASM interface and writes the QASM file, which is saved to be used by the
/// user on whatever platform they see fit. QASM input is widely supported on various quantum
/// computing platforms.
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    /// Name of the qubit_register assigned to the roqoqo qubits.
    ///
    /// roqoqo uses as unified address-space for qubits.
    /// There are no separate quantum registers and all qubits are addressed with `usize` addresses.
    /// When translating to QASM which uses explicitely declared qubit registers a name for the qubit
    /// register needs to be chosen.
    qubit_register_name: String,
}

impl Backend {
    /// Creates new QASM backend.
    ///
    /// # Arguments
    ///
    /// * `qubit_register_name` - The number of qubits in the backend.
    /// * ``
    ///
    pub fn new(qubit_register_name: Option<String>) -> Self {
        match qubit_register_name {
            None => Self {
                qubit_register_name: "q".to_string(),
            },
            Some(s) => Self {
                qubit_register_name: s,
            },
        }
    }
    /// Translates an iterator over operations to a valid QASM string.
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The iterator over [roqoqo::Operation] items that is translated
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The valid QASM string
    /// * `RoqoqoBackendError::OperationNotInBackend` - An operation is not available on the backend
    pub fn circuit_iterator_to_qasm_str<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
    ) -> Result<String, RoqoqoBackendError> {
        let mut data: String = "".to_string();
        let mut qasm_string = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\n");

        let mut number_qubits_required: usize = 0;
        for op in circuit {
            if let InvolvedQubits::Set(involved_qubits) = op.involved_qubits() {
                number_qubits_required =
                    number_qubits_required.max(match involved_qubits.iter().max() {
                        None => 0,
                        Some(n) => *n,
                    })
            }
            data.push_str(&call_operation(op, &self.qubit_register_name)?);
            data.push('\n');
        }
        qasm_string.push_str(
            format!(
                "qreg {}[{}];\n",
                self.qubit_register_name,
                number_qubits_required + 1
            )
            .as_str(),
        );

        qasm_string.push_str(data.as_str());

        Ok(qasm_string)
    }

    /// Translates an iterator over operations to a QASM file.
    ///
    /// # Arguments
    ///
    /// * `circuit` - The iterator over [roqoqo::Operation] items that is translated
    /// * `folder_name` - The name of the folder that is prepended to all filenames.
    /// * `filename` - The name of the file the QASM text is saved to.
    /// * `overwrite` - Whether to overwrite file if it already exists.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The qasm file was correctly written
    /// * `RoqoqoBackendError::FileAlreadyExists` - The file at this location already exists
    pub fn circuit_iterator_to_qasm_file<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
        folder_name: &Path,
        filename: &Path,
        overwrite: bool,
    ) -> Result<(), RoqoqoBackendError> {
        let data: String = self.circuit_iterator_to_qasm_str(circuit)?;

        let output_path: PathBuf = folder_name.join(filename.with_extension("qasm"));
        if output_path.is_file() && !overwrite {
            return Err(RoqoqoBackendError::FileAlreadyExists {
                path: output_path.to_str().unwrap().to_string(),
            });
        } else {
            let f = File::create(output_path).expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_str().as_bytes())
                .expect("Unable to write file")
        }

        Ok(())
    }

    /// Translates a Circuit to a valid QASM string.
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The Circuit items that is translated
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The valid QASM string
    /// * `RoqoqoBackendError::OperationNotInBackend` - An operation is not available on the backend
    pub fn circuit_to_qasm_str(&self, circuit: &Circuit) -> Result<String, RoqoqoBackendError> {
        self.circuit_iterator_to_qasm_str(circuit.iter())
    }

    /// Translates a Circuit to a QASM file.
    ///
    /// # Arguments
    ///
    /// * `circuit` - The Circuit that is translated
    /// * `folder_name` - The name of the folder that is prepended to all filenames.
    /// * `filename` - The name of the file the QASM text is saved to.
    /// * `overwrite` - Whether to overwrite file if it already exists.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The qasm file was correctly written
    /// * `RoqoqoBackendError::FileAlreadyExists` - The file at this location already exists
    pub fn circuit_to_qasm_file(
        &self,
        circuit: &Circuit,
        folder_name: &Path,
        filename: &Path,
        overwrite: bool,
    ) -> Result<(), RoqoqoBackendError> {
        self.circuit_iterator_to_qasm_file(circuit.iter(), folder_name, filename, overwrite)
    }
}
