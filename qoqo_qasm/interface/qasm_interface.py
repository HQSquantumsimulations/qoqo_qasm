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
    Tuple
)
from hqsbase.calculator import (
    Calculator,
    CalculatorFloat,
    CalculatorComplex
)
import numpy as np
from copy import deepcopy


# Create look-up tables

_QASM_ARGUMENT_NAME_DICTS: Dict[str, Dict[str, CalculatorFloat]] = dict()
_QASM_DEFAULT_EXPONENT: float = cast(float, None)
_QASM_NAME: Dict[str, str] = dict()

_QASM_ARGUMENT_NAME_DICTS['RotateZ'] = {'qubit': ('qubits', 'qubit'),
                                        'theta': ('parameters', 'theta')}
_QASM_NAME['RotateZ'] = 'rz'
_QASM_ARGUMENT_NAME_DICTS['RotateX'] = {'qubit': ('qubits', 'qubit'),
                                        'theta': ('parameters', 'theta')}
_QASM_NAME['RotateX'] = 'rx'
_QASM_ARGUMENT_NAME_DICTS['RotateY'] = {'qubit': ('qubits', 'qubit'),
                                        'theta': ('parameters', 'theta')}
_QASM_NAME['RotateY'] = 'ry'
_QASM_ARGUMENT_NAME_DICTS['CNOT'] = {'control': ('qubits', 'control'),
                                     'qubit': ('qubits', 'qubit')}
_QASM_NAME['CNOT'] = 'cx'
_QASM_ARGUMENT_NAME_DICTS['SingleQubitGate'] = {'qubit': ('qubits', 'qubit'),
                                                'alpha': ('parameters', [('alpha_r', 1),
                                                                         ('alpha_i', 1j)]),
                                                'beta': ('parameters', [('beta_r', 1),
                                                                        ('beta_i', 1j)])}
_QASM_NAME['SingleQubitGate'] = 'u3'
_QASM_ARGUMENT_NAME_DICTS['Hadamard'] = {'qubit': ('qubits', 'qubit')}
_QASM_NAME['Hadamard'] = 'h'
_QASM_ARGUMENT_NAME_DICTS['PauliX'] = {'qubit': ('qubits', 'qubit')}
_QASM_NAME['PauliX'] = 'x'
_QASM_ARGUMENT_NAME_DICTS['PauliY'] = {'qubit': ('qubits', 'qubit')}
_QASM_NAME['PauliY'] = 'y'
_QASM_ARGUMENT_NAME_DICTS['PauliZ'] = {'qubit': ('qubits', 'qubit')}
_QASM_NAME['PauliZ'] = 'z'
_QASM_ARGUMENT_NAME_DICTS['SGate'] = {'qubit': ('qubits', 'qubit')}
_QASM_NAME['SGate'] = 's'
_QASM_ARGUMENT_NAME_DICTS['TGate'] = {'qubit': ('qubits', 'qubit')}
_QASM_NAME['TGate'] = 't'
_QASM_ARGUMENT_NAME_DICTS['SqrtPauliX'] = {'qubit': ('qubits', 'qubit'),
                                           'theta': ('additional', str(np.pi / 2))}
_QASM_NAME['SqrtPauliX'] = 'rx'
_QASM_ARGUMENT_NAME_DICTS['MolmerSorensenXX'] = {'control': ('qubits', 'control'),
                                                 'qubit': ('qubits', 'qubit')}
_QASM_NAME['MolmerSorensenXX'] = 'rxx(pi/2)'
_QASM_ARGUMENT_NAME_DICTS['ControlledPauliY'] = {'control': ('qubits', 'control'),
                                                 'qubit': ('qubits', 'qubit')}
_QASM_NAME['ControlledPauliY'] = 'cy'
_QASM_ARGUMENT_NAME_DICTS['ControlledPauliZ'] = {'control': ('qubits', 'control'),
                                                 'qubit': ('qubits', 'qubit')}
_QASM_NAME['ControlledPauliZ'] = 'cz'

_QASM_NAME['MeasureQubit'] = 'measure'
_QASM_NAME['Definition'] = 'creg'

_ALLOWED_PRAGMAS = ['PragmaSetNumberOfMeasurements']


# Defining the actual call

def qasm_call_circuit(
        circuit: Circuit,
        calculator: Optional[Calculator] = None,
        number_qubits: Optional[int] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        use_symbolic: bool = False,
        **kwargs) -> Tuple[List[str], Dict[int, str]]:
    """Translate the qoqo circuit into QASM ouput

    The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
    to QASM output (strings).

    Args:
        circuit: The qoqo circuit that is translated
        calculator: The HQSBase Calculator used to replace symbolic parameters
        number_qubits: The number of qubits in the circuit
        qubit_names: The dictionary of qubit names to translate the circuit to
        use_symbolic: Whether to use symbolic translation (True) or not (False)
        **kwargs: Additional keyword arguments

    Returns:
        Tuple[List[str], Dict[int, str]]: translated circuit
    """
    lines = []

    if use_symbolic is True:
        symbolic_hash_lib: Dict[int, str] = dict()
        for op in circuit:
            tags: Tuple = op._operation_tags
            if 'Definition' in tags:
                op = cast(ops.Definition, op)
                append_operation = qasm_call_operation(op,
                                                       calculator,
                                                       number_qubits,
                                                       qubit_names,
                                                       **kwargs)
                if op._is_output:
                    lines.append(append_operation + ';')
            else:
                if 'RotateX' in tags or 'RotateY' in tags or 'RotateZ' in tags:
                    if op.is_parameterized:
                        op2 = deepcopy(op)
                        symbolic_hash = hash(op._ordered_parameter_dict['theta'].value)
                        symbolic_hash_lib[symbolic_hash] = op._ordered_parameter_dict['theta'].value
                        op2._ordered_parameter_dict['theta'] = CalculatorFloat(symbolic_hash)
                        append_operation = qasm_call_operation(op,
                                                               calculator,
                                                               number_qubits,
                                                               qubit_names,
                                                               **kwargs)
                else:
                    append_operation = qasm_call_operation(op,
                                                           calculator,
                                                           number_qubits,
                                                           qubit_names,
                                                           **kwargs)
                if append_operation is not None:
                    lines.append(append_operation + ';')

    else:
        symbolic_hash_lib = cast(Dict[int, str], None)
        for op in circuit:
            tags = op._operation_tags
            if 'Definition' in tags:
                op = cast(ops.Definition, op)
                append_operation = qasm_call_operation(op,
                                                       calculator,
                                                       number_qubits,
                                                       qubit_names,
                                                       **kwargs)
                if op._is_output:
                    lines.append(append_operation + ';')
            else:
                append_operation = qasm_call_operation(op,
                                                       calculator,
                                                       number_qubits,
                                                       qubit_names,
                                                       **kwargs)
                if append_operation is not None:
                    lines.append(append_operation + ';')

    return lines, symbolic_hash_lib


def qasm_call_operation(
        operation: ops.Operation,
        calculator: Optional[Calculator] = None,
        number_qubits: Optional[int] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        **kwargs) -> str:
    """Translate a qoqo operation to QASM text

    Args:
        operation: The qoqo operation that is translated
        calculator: The HQSBase Calculator used to replace symbolic parameters
        number_qubits: The number of qubits in the circuit
        qubit_names: The dictionary of qubit names to translate the operation to
        **kwargs: Additional keyword arguments

    Returns:
        str: translated operation

    Raises:
        OperationNotInBackendError: Operation not in QASM backend
    """
    op = cast(str, None)
    tags = operation._operation_tags
    if 'RotateZ' in tags:
        op = _execute_GateOperation(
            operation, 'RotateZ', calculator, qubit_names, **kwargs)
    elif 'RotateX' in tags:
        op = _execute_GateOperation(
            operation, 'RotateX', calculator, qubit_names, **kwargs)
    elif 'RotateY' in tags:
        op = _execute_GateOperation(
            operation, 'RotateY', calculator, qubit_names, **kwargs)
    elif 'CNOT' in tags:
        op = _execute_GateOperation(
            operation, 'CNOT', calculator, qubit_names, **kwargs)
    elif 'Hadamard' in tags:
        op = _execute_GateOperation(
            operation, 'Hadamard', calculator, qubit_names, **kwargs)
    elif 'PauliX' in tags:
        op = _execute_GateOperation(
            operation, 'PauliX', calculator, qubit_names, **kwargs)
    elif 'PauliY' in tags:
        op = _execute_GateOperation(
            operation, 'PauliY', calculator, qubit_names, **kwargs)
    elif 'PauliZ' in tags:
        op = _execute_GateOperation(
            operation, 'PauliZ', calculator, qubit_names, **kwargs)
    elif 'SGate' in tags:
        op = _execute_GateOperation(
            operation, 'SGate', calculator, qubit_names, **kwargs)
    elif 'TGate' in tags:
        op = _execute_GateOperation(
            operation, 'TGate', calculator, qubit_names, **kwargs)
    elif 'SqrtPauliX' in tags:
        op = _execute_GateOperation(
            operation, 'SqrtPauliX', calculator, qubit_names, **kwargs)
    elif 'MolmerSorensenXX' in tags:
        op = _execute_GateOperation(
            operation, 'MolmerSorensenXX', calculator, qubit_names, **kwargs)
    elif 'ControlledPauliY' in tags:
        op = _execute_GateOperation(
            operation, 'ControlledPauliY', calculator, qubit_names, **kwargs)
    elif 'ControlledPauliZ' in tags:
        op = _execute_GateOperation(
            operation, 'ControlledPauliZ', calculator, qubit_names, **kwargs)
    elif 'SingleQubitGate' in tags:
        op = _execute_SingleQubitGate(
            operation, calculator, number_qubits, qubit_names, **kwargs)
    elif 'PragmaRepeatedMeasurement' in tags:
        op = _execute_PragmaRepeatedMeasurement(
            operation, calculator, qubit_names, **kwargs)
    elif 'MeasureQubit' in tags:
        op = _execute_MeasureQubit(
            operation, calculator, qubit_names, **kwargs)
    elif 'Definition' in tags:
        op = _execute_Define(
            operation, calculator, qubit_names, **kwargs)
    elif any(pragma in tags for pragma in _ALLOWED_PRAGMAS):
        pass
    else:
        raise ops.OperationNotInBackendError('Operation not in QASM backend')

    return op


def _execute_SingleQubitGate(
        operation: ops.Operation,
        calculator: Optional[Calculator] = None,
        number_qubits: Optional[int] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        **kwargs) -> str:
    operation = cast(ops.SingleQubitGate, operation)
    parameter_dict: Dict[str, CalculatorFloat] = dict()
    if calculator is not None:
        for key, sarg in operation._ordered_parameter_dict.items():
            parameter_dict[key] = (calculator.parse_get(sarg.value))
    else:
        for key, sarg in operation._ordered_parameter_dict.items():
            parameter_dict[key] = sarg.value

    alpha = CalculatorComplex.from_pair(parameter_dict['alpha_r'], parameter_dict['alpha_i'])
    beta = CalculatorComplex.from_pair(parameter_dict['beta_r'], parameter_dict['beta_i'])
    abs_alpha = alpha.__abs__()
    angle_alpha = -1 * alpha.arg()
    angle_beta = beta.arg()

    theta = 2 * abs_alpha.acos()
    phi = angle_alpha + angle_beta
    lamda = angle_alpha - angle_beta

    if qubit_names is None:
        qubit = 'q[{}]'.format(operation._ordered_qubits_dict['qubit'])
    else:
        qubit = qubit_names[operation._ordered_qubits_dict['qubit']]

    gate_string = 'u3({},'.format(theta)
    gate_string += '{},'.format(phi)
    gate_string += '{})'.format(lamda)
    gate_string += ' {}'.format(qubit)

    return gate_string


def _execute_PragmaRepeatedMeasurement(
        operation: ops.Operation,
        calculator: Optional[Calculator] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        **kwargs) -> str:
    operation = cast(ops.PragmaRepeatedMeasurement, operation)
    if qubit_names is not None:
        meas = ''
        for key, val in qubit_names.items():
            meas += _QASM_NAME['MeasureQubit'] + ' q[{}]'.format(key)
            meas += ' -> {}[{}]\n'.format(operation._readout,
                                          val)
    else:
        meas = _QASM_NAME['MeasureQubit']
        meas += ' q'
        meas += ' -> {}'.format(operation._readout)
    return meas


def _execute_MeasureQubit(
        operation: ops.Operation,
        calculator: Optional[Calculator] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        **kwargs) -> str:
    operation = cast(ops.MeasureQubit, operation)
    if qubit_names is not None:
        qubit = qubit_names[operation._qubit]
    else:
        qubit = 'q[{}]'.format(operation._qubit)

    meas = _QASM_NAME['MeasureQubit']
    meas += ' {}'.format(qubit)
    meas += ' -> {}[{}]'.format(operation._readout,
                                operation._readout_index)
    return meas


def _execute_Define(
        operation: ops.Operation,
        calculator: Optional[Calculator] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        **kwargs) -> str:
    operation = cast(ops.Definition, operation)
    definition = "creg {name}[{length}]".format(name=operation._name, length=operation._length)
    return definition


def _execute_GateOperation(
        operation: ops.Operation,
        tag: str,
        calculator: Optional[Calculator] = None,
        qubit_names: Optional[Dict[int, str]] = None,
        **kwargs) -> str:
    operation = cast(ops.GateOperation, operation)
    name = _QASM_NAME[tag]
    qubits: List[str] = list()
    qkwargs: Dict[str, float] = dict()

    if qubit_names is None:
        qubit_names = {i: 'q[{}]'.format(i) for i in operation._ordered_qubits_dict.values()}

    parameter_dict: Dict[str, CalculatorFloat] = dict()
    if calculator is not None:
        for key, sarg in operation._ordered_parameter_dict.items():
            parameter_dict[key] = calculator.parse_get(sarg.value)
    else:
        for key, sarg in operation._ordered_parameter_dict.items():
            parameter_dict[key] = sarg.value

    for key in _QASM_ARGUMENT_NAME_DICTS[tag].keys():
        dict_name, dict_key = _QASM_ARGUMENT_NAME_DICTS[tag][key]
        if dict_name == 'qubits':
            arg = operation._ordered_qubits_dict[dict_key]
            if qubit_names is not None:
                sarg = qubit_names[arg]
            qubits.append(sarg)
        elif dict_name == 'parameters':
            parg = parameter_dict[dict_key]
            qkwargs[key] = parg
        elif dict_name == 'additional':
            qkwargs[key] = cast(float, dict_key)

    # Parameters
    if len(qkwargs.keys()) > 0:
        name += '('
        for co, key in enumerate(qkwargs.keys()):
            name += '{}'.format(qkwargs[key])
            if co < len(qkwargs.keys()) - 1:
                name += ','
        name += ')'

    # Qubits
    name += ' '
    for co, qubit in enumerate(qubits):
        name += '{}'.format(qubit)
        if co < len(qubits) - 1:
            name += ','

    return name
