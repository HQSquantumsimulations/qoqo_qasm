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
from qoqo.backends import (
    BackendBaseClass,
)
from qoqo import Circuit
import os
from hqsbase.calculator import Calculator
from typing import (
    Optional,
    Dict,
    Set,
    Iterable,
    List,
    cast
)
from qoqo.devices import DeviceBaseClass
from hqsbase.qonfig import Qonfig, empty
from qoqo_qasm import qasm_call_circuit


class QasmBackend(BackendBaseClass):
    r"""Backend to qoqo that produces QASM output which can be imported.

    This backend takes a qoqo circuit to be run on a certain device and returns a QASM file
    containing the translated circuit. The circuit itself is translated using the qoqo_qasm
    interface. In this backend, the initialization sets up the relevant parameters and the run
    function calls the QASM interface and writes the QASM file, which is saved to be used by the
    user on whatever platform they see fit. QASM input is widely supported on various quantum
    computing platforms.
    """

    _qonfig_defaults_dict = {
        'circuit': {'doc': 'The circuit that is run',
                    'default': None},
        'number_qubits': {'doc': 'The number of qubits to use',
                          'default': empty},
        'substitution_dict': {'doc': 'Substitution dictionary used to replace symbolic parameters',
                              'default': None},
        'number_measurements': {'doc': 'The number of measurement repetitions',
                                'default': 1},
        'device': {'doc': 'The device specification',
                   'default': None},
        'folder_name': {'doc': 'Name of the folder that is prepended all filenames in run',
                        'default': ''},
        'qureg_name': {'doc': 'Name of the qubit register',
                       'default': 'q'},
    }

    def __init__(self,
                 circuit: Optional[Circuit] = None,
                 number_qubits: int = 1,
                 substitution_dict: Optional[Dict[str, float]] = None,
                 number_measurements: int = 1,
                 device: Optional[DeviceBaseClass] = None,
                 folder_name: str = '',
                 qureg_name: str = 'q',
                 **kwargs) -> None:
        """Initialize QASM Backend

        Args:
            circuit: The circuit that is run
            number_qubits: The number of qubits to use
            substitution_dict: Substitution dictionary used to replace symbolic parameters
            number_measurements: The number of measurement repetitions
            device: The device specification
            folder_name: The name of the folder that is prepended to all filenames in run function
            qureg_name: The name of the qubit register
            kwargs: Additional keyword arguments

        """
        self.name = "qasm"

        self.number_qubits = number_qubits
        self.substitution_dict = substitution_dict
        self.number_measurements = number_measurements
        self.device = device
        self.folder_name = folder_name
        self.qureg_name = qureg_name
        self.kwargs = kwargs
        self.set_of_parameters: Set[str] = set()
        self.qubit_names = getattr(self.device, '_qubit_names', None)

        if self.substitution_dict is None:
            self.calculator = None
        else:
            self.calculator = Calculator()
            for name, val in self.substitution_dict.items():
                self.calculator.set(name, val)

        super().__init__(circuit=circuit,
                         substitution_dict=self.substitution_dict,
                         device=self.device,
                         number_qubits=number_qubits,
                         **kwargs)

        if circuit is None:
            circuit = Circuit()
        self.compiled_circuit, _ = qasm_call_circuit(circuit=circuit,
                                                     calculator=self.calculator,
                                                     number_qubits=self.number_qubits,
                                                     qubit_names=self.qubit_names,
                                                     use_symbolic=False)

    @classmethod
    def from_qonfig(cls,
                    config: Qonfig['QasmBackend']
                    ) -> 'QasmBackend':
        """Create an Instance from Qonfig

        Args:
            config: Qonfig of class

        Returns:
            QasmBackend
        """
        if isinstance(config['circuit'], Qonfig):
            init_circuit = config['circuit'].to_instance()
        else:
            init_circuit = cast(Optional[Circuit], config['circuit'])
        if isinstance(config['device'], Qonfig):
            init_device = config['device'].to_instance()
        else:
            init_device = cast(Optional[DeviceBaseClass], config['device'])
        return cls(circuit=init_circuit,
                   number_qubits=config['number_qubits'],
                   substitution_dict=config['substitution_dict'],
                   number_measurements=config['number_measurements'],
                   device=init_device,
                   folder_name=config['folder_name'],
                   qureg_name=config['qureg_name'],
                   )

    def to_qonfig(self) -> 'Qonfig[QasmBackend]':
        """Create a Qonfig from Instance

        Returns:
            Qonfig[QasmBackend]
        """
        config = Qonfig(self.__class__)
        if self._circuit is not None:
            config['circuit'] = self._circuit.to_qonfig()
        else:
            config['circuit'] = self._circuit
        config['number_qubits'] = self.number_qubits
        config['substitution_dict'] = self.substitution_dict
        config['number_measurements'] = self.number_measurements
        if self.device is not None:
            config['device'] = self.device.to_qonfig()
        else:
            config['device'] = self.device
        config['folder_name'] = self.folder_name
        config['qureg_name'] = self.qureg_name

        return config

    def run(self, filename: str = 'default_qasm_backend_output', overwrite: bool = False,
            use_symbolic: bool = False,
            **kwargs) -> None:
        """Turn the circuit into QASM and save to file

        Args:
            filename: The name of the file the QASM text is saved to
            overwrite: Whether to overwrite file if it already exists, defaulting to False
            use_symbolic: Whether to use symbolic parameters, defaulting to False
            kwargs: Additional keyword arguments

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
        output_lines.append('include "qelib.inc";\n\n')

        qasm_qubit_names: Dict[int, str] = dict()
        for ci in range(self.number_qubits):
            if self.qubit_names is None:
                qasm_qubit_names[ci] = '{}[{}]'.format(self.qureg_name, ci)
            else:
                qasm_qubit_names[ci] = '{}[{}]'.format(self.qureg_name, self.qubit_names[ci])
        output_lines.append('qreg {}[{}];\n'.format(self.qureg_name, self.number_qubits))

        if use_symbolic:
            circuit_lines, self.symbolic_hash_lib = qasm_call_circuit(
                circuit=self.circuit,
                calculator=self.calculator,
                number_qubits=self.number_qubits,
                qubit_names=qasm_qubit_names,
                use_symbolic=True)
        else:
            circuit_lines, _ = qasm_call_circuit(
                circuit=self.circuit,
                calculator=self.calculator,
                number_qubits=self.number_qubits,
                qubit_names=qasm_qubit_names,
                use_symbolic=False)

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
