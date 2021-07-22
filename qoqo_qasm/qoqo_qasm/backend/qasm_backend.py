"""Backend producing QASM"""
# Copyright © 2019-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
from qoqo import Circuit
import os
from typing import (
    Optional,
    Dict,
    Iterable,
    List,
)
from qoqo_qasm import qasm_call_circuit


class QasmBackend(object):
    r"""Backend to qoqo that produces QASM output which can be imported.

    This backend takes a qoqo circuit to be run on a certain device and returns a QASM file
    containing the translated circuit. The circuit itself is translated using the qoqo_qasm
    interface. In this backend, the initialization sets up the relevant parameters and the run
    function calls the QASM interface and writes the QASM file, which is saved to be used by the
    user on whatever platform they see fit. QASM input is widely supported on various quantum
    computing platforms.
    """

    def __init__(self,
                 number_qubits: int = 1,
                 folder_name: str = '',
                 qureg_name: str = 'q',
                 qubit_names: Optional[Dict[int, str]] = None) -> None:
        """Initialize QASM Backend

        Args:
            number_qubits: The number of qubits to use
            folder_name: The name of the folder that is prepended to all filenames in run function
            qureg_name: The name of the qubit register
            qubit_names: The dictionary of qubit names to translate the circuit to

        """
        self.name = "qasm"
        self.number_qubits = number_qubits
        self.folder_name = folder_name
        self.qureg_name = qureg_name
        self.qubit_names = qubit_names

    def run_circuit(self, circuit: Circuit, filename: str = 'default_qasm_backend_output',
                    overwrite: bool = False, use_symbolic: bool = False) -> None:
        """Turn the circuit into QASM and save to file

        Args:
            circuit: Circuit to be run
            filename: The name of the file the QASM text is saved to
            overwrite: Whether to overwrite file if it already exists, defaulting to False
            use_symbolic: Whether to use symbolic parameters, defaulting to False

        Raises:
            FileExistsError: Qasm file already exists, aborting.
                             Use overwrite=True to replace the existing file.

        """
        filename = os.path.join(os.path.abspath(os.path.expanduser(self.folder_name)),
                                filename + '.qasm')
        filedir = os.path.dirname(os.path.expanduser(filename))
        os.makedirs(filedir, exist_ok=True)
        if os.path.isfile(filename) and overwrite is not True:
            raise FileExistsError(
                "Python file {} already exists, aborting. User overwrite to replace".format(
                    filename))
        output_lines = list()
        output_lines.append('OPENQASM 2.0;\n')
        output_lines.append('include "qelib1.inc";\n\n')

        qasm_qubit_names: Dict[int, str] = dict()
        for ci in range(self.number_qubits):
            if self.qubit_names is None:
                qasm_qubit_names[ci] = '{}[{}]'.format(self.qureg_name, ci)
            else:
                qasm_qubit_names[ci] = '{}[{}]'.format(self.qureg_name, self.qubit_names[ci])
        output_lines.append('qreg {}[{}];\n'.format(self.qureg_name, self.number_qubits))

        if use_symbolic:
            circuit_lines, self.symbolic_hash_lib = qasm_call_circuit(
                circuit=circuit,
                number_qubits=self.number_qubits,
                qubit_names=qasm_qubit_names,
                use_symbolic=True)
        else:
            circuit_lines, _ = qasm_call_circuit(
                circuit=circuit,
                number_qubits=self.number_qubits,
                qubit_names=qasm_qubit_names,
                use_symbolic=False)
        self.circuit = circuit_lines

        output_lines.extend(
            _append_newlines(
                circuit_lines
            )
        )
        with open(filename, 'w') as fo:
            fo.writelines(output_lines)


def _append_newlines(seq: Iterable[str]) -> List[str]:
    return_seq = list()
    for s in seq:
        if s is not None:
            if s not in ["", ";"]:
                return_seq.append(s + '\n')
    return return_seq
