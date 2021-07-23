"""Define the QASM interface for qoqo operations."""
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

from qoqo import operations as ops
from qoqo import Circuit
from typing import (
    Optional,
    Dict,
    cast,
    List,
    Tuple,
    Any
)
from qoqo_calculator_pyo3 import (
    CalculatorFloat,
    CalculatorComplex
)
import numpy as np

_ALLOWED_PRAGMAS = ['PragmaSetNumberOfMeasurements', 'InputSymbolic']


def qasm_call_circuit(
        circuit: Circuit,
        number_qubits: Optional[int] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        use_symbolic: bool = False) -> Tuple[List[str], Dict[int, str]]:
    """Translate the qoqo circuit into QASM ouput

    The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
    to QASM output (strings).

    Args:
        circuit: The qoqo circuit that is translated
        number_qubits: The number of qubits in the circuit
        qubit_names: The dictionary of qubit names to translate the circuit to
        use_symbolic: Whether to use symbolic translation (True) or not (False)

    Returns:
        Tuple[List[str], Dict[int, str]]: translated circuit
    """
    lines = []

    if use_symbolic is True:
        symbolic_hash_lib: Dict[int, str] = dict()
        for op in circuit:
            tags: Tuple = op.tags()
            if 'Definition' in tags:
                append_operation = qasm_call_operation(op,
                                                       number_qubits,
                                                       qubit_names)
                if append_operation is not None:
                    lines.append(append_operation + ';')
            else:
                if 'RotateX' in tags or 'RotateY' in tags or 'RotateZ' in tags:
                    if op.is_parametrized():
                        symbolic_hash = hash(op.theta())
                        symbolic_hash_lib[symbolic_hash] = op.theta()
                        if 'RotateX' in tags:
                            op2 = ops.RotateX(op.qubit(), CalculatorFloat(symbolic_hash))
                        elif 'RotateY' in tags:
                            op2 = ops.RotateY(op.qubit(), CalculatorFloat(symbolic_hash))
                        elif 'RotateZ' in tags:
                            op2 = ops.RotateZ(op.qubit(), CalculatorFloat(symbolic_hash))
                        append_operation = qasm_call_operation(op2,
                                                               number_qubits,
                                                               qubit_names)
                else:
                    append_operation = qasm_call_operation(op,
                                                           number_qubits,
                                                           qubit_names)
                if append_operation is not None:
                    lines.append(append_operation + ';')

    else:
        symbolic_hash_lib = cast(Dict[int, str], None)
        for op in circuit:
            tags = op.tags()
            if 'Definition' in tags:
                append_operation = qasm_call_operation(op,
                                                       number_qubits,
                                                       qubit_names)
                if append_operation is not None:
                    lines.append(append_operation + ';')
            else:
                append_operation = qasm_call_operation(op,
                                                       number_qubits,
                                                       qubit_names)
                # Exclude final ; for PragmaRepeatedMeasurement
                # because it can produce multi-line statement
                if "PragmaRepeatedMeasurement" in tags:
                    lines.append(append_operation)
                elif append_operation is not None:
                    lines.append(append_operation + ';')

    return lines, symbolic_hash_lib


def qasm_call_operation(
        operation: Any,
        number_qubits: Optional[int] = None,
        qubit_names: Optional[Dict[int, str]] = None,) -> str:
    """Translate a qoqo operation to QASM text

    Args:
        operation: The qoqo operation that is translated
        number_qubits: The number of qubits in the circuit
        qubit_names: The dictionary of qubit names to translate the operation to

    Returns:
        str: translated operation

    Raises:
        RuntimeError: Operation not in QASM backend
    """
    op = cast(str, None)
    tags = operation.tags()

    if 'RotateZ' in tags:
        operation = cast(ops.RotateZ, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'rz({theta}) {qubit}'.format(theta=float(operation.theta()), qubit=qubit)
    elif 'RotateX' in tags:
        operation = cast(ops.RotateX, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'rx({theta}) {qubit}'.format(theta=float(operation.theta()), qubit=qubit)
    elif 'RotateY' in tags:
        operation = cast(ops.RotateY, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'ry({theta}) {qubit}'.format(theta=float(operation.theta()), qubit=qubit)
    elif 'CNOT' in tags:
        operation = cast(ops.CNOT, operation)
        control = 'q[{}]'.format(
            operation.control()) if qubit_names is None else qubit_names[operation.control()]
        qubit = 'q[{}]'.format(
            operation.target()) if qubit_names is None else qubit_names[operation.target()]
        op = 'cx {control},{qubit}'.format(control=control, qubit=qubit)
    elif 'Hadamard' in tags:
        operation = cast(ops.Hadamard, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'h {qubit}'.format(qubit=qubit)
    elif 'PauliX' in tags:
        operation = cast(ops.PauliX, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'x {qubit}'.format(qubit=qubit)
    elif 'PauliY' in tags:
        operation = cast(ops.PauliY, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'y {qubit}'.format(qubit=qubit)
    elif 'PauliZ' in tags:
        operation = cast(ops.PauliZ, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'z {qubit}'.format(qubit=qubit)
    elif 'SGate' in tags:
        operation = cast(ops.SGate, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 's {qubit}'.format(qubit=qubit)
    elif 'TGate' in tags:
        operation = cast(ops.TGate, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 't {qubit}'.format(qubit=qubit)
    elif 'SqrtPauliX' in tags:
        operation = cast(ops.SqrtPauliX, operation)
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'rx({theta}) {qubit}'.format(theta=str(np.pi / 2), qubit=qubit)
    elif 'MolmerSorensenXX' in tags:
        operation = cast(ops.MolmerSorensenXX, operation)
        control = 'q[{}]'.format(
            operation.control()) if qubit_names is None else qubit_names[operation.control()]
        qubit = 'q[{}]'.format(
            operation.target()) if qubit_names is None else qubit_names[operation.target()]
        op = 'rxx(pi/2) {control},{qubit}'.format(control=control, qubit=qubit)
    elif 'ControlledPauliY' in tags:
        operation = cast(ops.ControlledPauliY, operation)
        control = 'q[{}]'.format(
            operation.control()) if qubit_names is None else qubit_names[operation.control()]
        qubit = 'q[{}]'.format(
            operation.target()) if qubit_names is None else qubit_names[operation.target()]
        op = 'cy {control},{qubit}'.format(control=control, qubit=qubit)
    elif 'ControlledPauliZ' in tags:
        operation = cast(ops.ControlledPauliZ, operation)
        control = 'q[{}]'.format(
            operation.control()) if qubit_names is None else qubit_names[operation.control()]
        qubit = 'q[{}]'.format(
            operation.target()) if qubit_names is None else qubit_names[operation.target()]
        op = 'cz {control},{qubit}'.format(control=control, qubit=qubit)
    elif 'SingleQubitGate' in tags:
        operation = cast(ops.SingleQubitGate, operation)
        op = _execute_SingleQubitGate(operation, qubit_names)
    elif 'PragmaRepeatedMeasurement' in tags:
        op = _execute_PragmaRepeatedMeasurement(operation, qubit_names)
    elif 'MeasureQubit' in tags:
        qubit = 'q[{}]'.format(
            operation.qubit()) if qubit_names is None else qubit_names[operation.qubit()]
        op = 'measure {qubit} -> {readout}[{readout_index}]'.format(
            qubit=qubit, readout=operation.readout(), readout_index=operation.readout_index())
    elif any(pragma in tags for pragma in _ALLOWED_PRAGMAS):
        pass
    elif 'Definition' in tags:
        op = 'creg {name}[{length}]'.format(name=operation.name(), length=operation.length())
    else:
        raise RuntimeError('Operation not in QASM backend')

    return op


def _execute_SingleQubitGate(
        operation: Any,
        qubit_names: Optional[Dict[int, str]] = None) -> str:
    operation = cast(ops.SingleQubitGate, operation)

    alpha = CalculatorComplex.from_pair(operation.alpha_r(), operation.alpha_i())
    beta = CalculatorComplex.from_pair(operation.beta_r(), operation.beta_i())
    abs_alpha = alpha.__abs__()
    angle_alpha = -1 * alpha.arg()
    angle_beta = beta.arg()

    theta = 2 * abs_alpha.acos()
    phi = angle_alpha + angle_beta
    lamda = angle_alpha - angle_beta

    if qubit_names is None:
        qubit = 'q[{}]'.format(operation.qubit())
    else:
        qubit = qubit_names[operation.qubit()]

    gate_string = 'u3({},'.format(float(theta))
    gate_string += '{},'.format(float(phi))
    gate_string += '{})'.format(float(lamda))
    gate_string += ' {}'.format(qubit)

    return gate_string


def _execute_PragmaRepeatedMeasurement(
        operation: Any,
        qubit_names: Optional[Dict[int, str]] = None,) -> str:
    operation = cast(ops.PragmaRepeatedMeasurement, operation)
    mapping_dictionary = operation.qubit_mapping()
    if qubit_names is not None and mapping_dictionary is not None:
        meas = ''
        for key in qubit_names.keys():
            meas += 'measure' + ' {}'.format(qubit_names[mapping_dictionary[key]])
            meas += ' -> {}[{}];\n'.format(operation.readout(),
                                           key)
    elif qubit_names is not None and mapping_dictionary is None:
        meas = ''
        for key in qubit_names.keys():
            meas += 'measure' + ' {}'.format(qubit_names[key])
            meas += ' -> {}[{}];\n'.format(operation.readout(),
                                           key)
    elif qubit_names is None and mapping_dictionary is not None:
        meas = ''
        for j in range(max(mapping_dictionary.keys()) + 1):
            meas += 'measure' + ' q[{}]'.format(mapping_dictionary[j])
            meas += ' -> {}[{}];\n'.format(operation.readout(),
                                           j)
    else:
        meas = 'measure'
        meas += ' q'
        meas += ' -> {};\n'.format(operation.readout())
    return meas
