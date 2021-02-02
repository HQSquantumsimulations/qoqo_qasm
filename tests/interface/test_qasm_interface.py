"""Test QASM interface"""
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
from qoqo import operations as ops
from qoqo import Circuit
from typing import (
    Dict,
    Any
)
from qoqo_qasm import qasm_call_circuit, qasm_call_operation
from qoqo.registers import BitRegisterOutput


@pytest.mark.parametrize("gate", [
    (ops.RotateX, 'rx(-3.141592653589793) q[0]'),
    (ops.RotateY, 'ry(-3.141592653589793) q[0]'),
    (ops.RotateZ, 'rz(-3.141592653589793) q[0]'),
    (ops.CNOT, 'cx q[0],q[1]'),
    (ops.Hadamard, 'h q[0]'),
    (ops.PauliX, 'x q[0]'),
    (ops.PauliY, 'y q[0]'),
    (ops.PauliZ, 'z q[0]'),
    (ops.SGate, 's q[0]'),
    (ops.TGate, 't q[0]'),
    (ops.SqrtPauliX, 'rx(1.5707963267948966) q[0]'),
    (ops.MolmerSorensenXX, 'rxx(pi/2) q[0],q[1]'),
    (ops.ControlledPauliY, 'cy q[0],q[1]'),
    (ops.ControlledPauliZ, 'cz q[0],q[1]'),
    (ops.SingleQubitGate, 'u3(0e0,0e0,0e0) q[0]'),
    (ops.MeasureQubit, 'measure q[0] -> ro[0]'),
    (ops.PragmaRepeatedMeasurement, 'measure q -> ro'),
    (ops.Definition, 'creg ro[1]')
])
def test_gate_translation(gate):
    """Test gate operations with QASM interface"""
    op = gate[0]
    op_string = gate[1]
    tags = op._operation_tags
    args = {'theta': -np.pi, 'alpha_r': 1, 'alpha_i': 0, 'beta_r': 0, 'beta_i': 0}

    if 'SingleQubitGateOperation' in tags:
        operation = op(qubit=0, **args)
    elif 'TwoQubitGateOperation' in tags:
        operation = op(control=0, qubit=1, **args)
    elif 'MeasureQubit' in tags:
        operation = op()
    elif 'Definition' in tags:
        operation = op('ro')
    elif 'PragmaRepeatedMeasurement' in tags:
        operation = op(readout='ro', number_measurements=1)

    qasm_operation = qasm_call_operation(operation=operation,
                                            calculator=None,
                                            number_qubits=2)

    npt.assert_string_equal(qasm_operation, op_string)



if __name__ == '__main__':
    pytest.main(sys.argv)