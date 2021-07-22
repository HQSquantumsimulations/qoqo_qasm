"""Test qoqo QASM interface"""
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
from qoqo_qasm import qasm_call_operation


@pytest.mark.parametrize("gate", [
    (ops.RotateX(0, -np.pi), 'rx(-3.141592653589793) q[0]'),
    (ops.RotateY(0, -np.pi), 'ry(-3.141592653589793) q[0]'),
    (ops.RotateZ(0, -np.pi), 'rz(-3.141592653589793) q[0]'),
    (ops.CNOT(0, 1), 'cx q[0],q[1]'),
    (ops.Hadamard(0), 'h q[0]'),
    (ops.PauliX(0), 'x q[0]'),
    (ops.PauliY(0), 'y q[0]'),
    (ops.PauliZ(0), 'z q[0]'),
    (ops.SGate(0), 's q[0]'),
    (ops.TGate(0), 't q[0]'),
    (ops.SqrtPauliX(0), 'rx(1.5707963267948966) q[0]'),
    (ops.MolmerSorensenXX(0, 1), 'rxx(pi/2) q[0],q[1]'),
    (ops.ControlledPauliY(0, 1), 'cy q[0],q[1]'),
    (ops.ControlledPauliZ(0, 1), 'cz q[0],q[1]'),
    (ops.SingleQubitGate(0, 1, 0, 1, 0, 1.0), 'u3(0.0,0.0,-0.0) q[0]'),
    (ops.PragmaRepeatedMeasurement('ro', None, 1), 'measure q -> ro;\n'),
    (ops.PragmaRepeatedMeasurement('ro', {0: 1, 1: 0}, 1),
     'measure q[1] -> ro[0];\nmeasure q[0] -> ro[1];\n'),
    (ops.MeasureQubit(0, 'ro', 0), 'measure q[0] -> ro[0]'),
    (ops.DefinitionFloat(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.DefinitionUsize(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.DefinitionBit(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.DefinitionComplex(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.InputSymbolic('other', 0), None),
    (ops.PragmaSetNumberOfMeasurements(20, 'ro'), None)
])
def test_gate_translation(gate: Tuple[Any, str]):
    """Test gate operations with QASM interface"""
    qasm_operation = qasm_call_operation(operation=gate[0],
                                         number_qubits=2)

    if gate[1] is None:
        assert qasm_operation is None
    else:
        npt.assert_string_equal(qasm_operation, gate[1])


@pytest.mark.parametrize("gate", [
    (ops.RotateX(0, -np.pi), 'rx(-3.141592653589793) s[1]'),
    (ops.RotateY(0, -np.pi), 'ry(-3.141592653589793) s[1]'),
    (ops.RotateZ(0, -np.pi), 'rz(-3.141592653589793) s[1]'),
    (ops.CNOT(0, 1), 'cx s[1],s[0]'),
    (ops.Hadamard(0), 'h s[1]'),
    (ops.PauliX(0), 'x s[1]'),
    (ops.PauliY(0), 'y s[1]'),
    (ops.PauliZ(0), 'z s[1]'),
    (ops.SGate(0), 's s[1]'),
    (ops.TGate(0), 't s[1]'),
    (ops.SqrtPauliX(0), 'rx(1.5707963267948966) s[1]'),
    (ops.MolmerSorensenXX(0, 1), 'rxx(pi/2) s[1],s[0]'),
    (ops.ControlledPauliY(0, 1), 'cy s[1],s[0]'),
    (ops.ControlledPauliZ(0, 1), 'cz s[1],s[0]'),
    (ops.SingleQubitGate(0, 1, 0, 1, 0, 1.0), 'u3(0.0,0.0,-0.0) s[1]'),
    (ops.PragmaRepeatedMeasurement('ro', {0: 0, 1: 1}, 1), 'measure s[1] -> ro[0];\nmeasure s[0] -> ro[1];\n'),
    (ops.PragmaRepeatedMeasurement('ro', {0: 1, 1: 0}, 1),
     'measure s[0] -> ro[0];\nmeasure s[1] -> ro[1];\n'),
    (ops.MeasureQubit(0, 'ro', 0), 'measure s[1] -> ro[0]'),
    (ops.DefinitionFloat(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.DefinitionUsize(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.DefinitionBit(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.DefinitionComplex(name='ro', length=1, is_output=True), 'creg ro[1]'),
    (ops.InputSymbolic('other', 0), None),
    (ops.PragmaSetNumberOfMeasurements(20, 'ro'), None)
])
def test_gate_translation_qubit_names(gate: Tuple[Any, str]):
    """Test gate operations with QASM interface"""
    qasm_operation = qasm_call_operation(operation=gate[0],
                                         number_qubits=2,
                                         qubit_names={0: "s[1]", 1: "s[0]"},)

    if gate[1] is None:
        assert qasm_operation is None
    else:
        npt.assert_string_equal(qasm_operation, gate[1])


if __name__ == '__main__':
    pytest.main(sys.argv)
