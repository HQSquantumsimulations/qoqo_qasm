"""Test qoqo QASM acceptance"""
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
import numpy as np
import numpy.testing as npt
from typing import Tuple, Any
from qoqo import operations as ops
from qoqo import Circuit
from qoqo_qasm import qasm_call_operation
from qoqo_qasm import QasmBackend
from qiskit import QuantumCircuit


def test_acceptance_with_qiskit():
    """Test gate operations with QASM interface"""
    circuit = Circuit()

    circuit += ops.RotateX(0, -np.pi)
    circuit += ops.RotateY(0, -np.pi)
    circuit += ops.RotateZ(0, -np.pi)
    circuit += ops.CNOT(0, 1)
    circuit += ops.Hadamard(0)
    circuit += ops.PauliX(0)
    circuit += ops.PauliY(0)
    circuit += ops.PauliZ(0)
    circuit += ops.SGate(0)
    circuit += ops.TGate(0)
    circuit += ops.SqrtPauliX(0)
    circuit += ops.MolmerSorensenXX(0, 1)
    circuit += ops.ControlledPauliY(0, 1)
    circuit += ops.ControlledPauliZ(0, 1)
    circuit += ops.SingleQubitGate(0, 1, 0, 1, 0, 1.0)
    circuit += ops.PragmaRepeatedMeasurement('ro', 1, None)
    circuit += ops.MeasureQubit(0, 'ro', 0)
    circuit += ops.DefinitionFloat(name='rof', length=1, is_output=True)
    circuit += ops.DefinitionBit(name='ro', length=2, is_output=True)
    circuit += ops.DefinitionComplex(name='roc', length=1, is_output=True)
    circuit += ops.InputSymbolic('other', 0)
    circuit += ops.PragmaSetNumberOfMeasurements(20, 'ro')

    backend = QasmBackend(number_qubits=2)
    backend.run_circuit(circuit=circuit, overwrite=True)

    q_circuit = QuantumCircuit.from_qasm_file("default_qasm_backend_output.qasm")


if __name__ == '__main__':
    pytest.main(sys.argv)
