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

use crate::call_operation;
use roqoqo::operations::*;
use roqoqo::{Circuit, RoqoqoBackendError};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// QASM backend to qoqo
///
/// This backend to qoqo produces QASM output which can be exported.
///
/// This backend takes a qoqo circuit to be run on a certain device and returns a QASM file
/// containing the translated circuit. The circuit itself is translated using the qoqo_qasm
/// interface. In this backend, the initialization sets up the relevant parameters and the run
/// function calls the QASM interface and writes the QASM file, which is saved to be used by the
/// user on whatever platform they see fit. QASM input is widely supported on various quantum
/// computing platforms.
///
/// # Arguments
///
/// * `number_qubits` - The number of qubits in the Backend.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Backend {
    number_qubits: usize,
}

impl Backend {
    /// Creates new QASM backend.
    ///
    /// # Arguments
    ///
    /// * `number_qubits` - The number of qubits in the backend.
    ///
    /// # Returns
    ///
    /// * `Backend` - The new instance of Backend with the specified inputs.
    pub fn new(number_qubits: usize) -> Self {
        Self { number_qubits }
    }
}


impl Backend {
    /// Runs a circuit with the backend.
    ///
    /// A circuit is passed to the roqoqo-qasm backend and executed.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    ///
    /// # Arguments
    ///
    /// * `circuit` - The circuit that is run on the backend.
    /// * `folder_name` - The name of the folder that is prepended to all filenames in run function.
    /// * `filename` - The name of the file the QASM text is saved to.
    /// * `overwrite` - Whether to overwrite file if it already exists.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - The qasm file was correctly written
    /// * `RoqoqoBackendError::FileAlreadyExists` - The file at this location already exists
    pub fn run_circuit_iterator<'a>(
        &self,
        circuit: impl Iterator<Item = &'a Operation>,
        folder_name: String,
        filename: String,
        overwrite: bool,
    ) -> Result<(), RoqoqoBackendError> {
        let mut data: String = String::from("OPENQASM 2.0\ninclude \"qelib1.inc\";\n\n");
        for op in circuit {
            if let Ok(x) = call_operation(op) {
                data.push_str(&x);
                data.push('\n');
            }
        }

        let mut path: String = folder_name;
        path.push_str(&filename);
        path.push_str(".qasm");
        if Path::new(path.as_str()).is_file() && !overwrite {
            return Err(RoqoqoBackendError::FileAlreadyExists { path });
        } else {
            let f = File::create(path).expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_str().as_bytes())
                .expect("Unable to write file")
        }

        Ok(())
    }

    /// Runs a circuit with the backend.
    ///
    /// A circuit is passed to the backend and executed.
    /// During execution values are written to and read from classical registers
    /// ([crate::registers::BitRegister], [crate::registers::FloatRegister] and [crate::registers::ComplexRegister]).
    /// To produce sufficient statistics for evaluating expectationg values,
    /// circuits have to be run multiple times.
    /// The results of each repetition are concatenated in OutputRegisters
    /// ([crate::registers::BitOutputRegister], [crate::registers::FloatOutputRegister] and [crate::registers::ComplexOutputRegister]).  
    ///
    ///
    /// # Arguments
    ///
    /// * `circuit` - The circuit that is run on the backend.
    /// * `folder_name` - The name of the folder that is prepended to all filenames in run function.
    /// * `filename` - The name of the file the QASM text is saved to.
    /// * `overwrite` - Whether to overwrite file if it already exists.
    ///
    /// # Returns
    ///
    /// `RegisterResult` - The output registers written by the evaluated circuits.
    pub fn run_circuit(
        &self,
        circuit: &Circuit,
        folder_name: String,
        filename: String,
        overwrite: bool,
    ) -> Result<(), RoqoqoBackendError> {
        self.run_circuit_iterator(circuit.iter(), folder_name, filename, overwrite)
    }
}
