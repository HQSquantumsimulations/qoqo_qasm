"""Testing qoqo QASM backend"""
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
import pytest
import sys
import numpy.testing as npt
from qoqo import operations as ops
from qoqo import Circuit
from qoqo_qasm import QasmBackend


def test_qasm_backend():
    """Testing the QASM functionality with the QASM backend"""
    circuit = Circuit()
    circuit += ops.DefinitionBit(name='ro', length=2, is_output=True)
    circuit += ops.RotateZ(qubit=0, theta=0)
    circuit += ops.PauliX(qubit=1)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index=0)
    circuit += ops.MeasureQubit(qubit=1, readout='ro', readout_index=1)

    backend = QasmBackend(number_qubits=2)

    backend.run_circuit(circuit=circuit, overwrite=True)
    circuit = ['creg ro[2];', 'rz(0.0) q[0];', 'x q[1];',
               'measure q[0] -> ro[0];', 'measure q[1] -> ro[1];']
    npt.assert_equal(backend.circuit, circuit)


def test_qasm_xx():
    """Testing the MolmerSorensenXX gate with the QASM backend"""
    circuit = Circuit()
    circuit += ops.DefinitionBit(name='ro', length=2, is_output=True)
    circuit += ops.MolmerSorensenXX(control=0, target=1)
    circuit += ops.MeasureQubit(qubit=0, readout='ro', readout_index=0)
    circuit += ops.MeasureQubit(qubit=1, readout='ro', readout_index=1)

    backend = QasmBackend(number_qubits=2)
    backend.run_circuit(circuit=circuit, overwrite=True)
    circuit = ['creg ro[2];', 'rxx(pi/2) q[0],q[1];',
               'measure q[0] -> ro[0];', 'measure q[1] -> ro[1];']
    npt.assert_equal(backend.circuit, circuit)


if __name__ == '__main__':
    pytest.main(sys.argv)
