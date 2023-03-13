# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for backend.py."""

import pytest
import sys

from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import ClassicalRegister, PauliZProduct, PauliZProductInput  # type:ignore
from qiskit.providers.fake_provider import FakeAthens

from qiskit_aer import AerSimulator

from qoqo_qiskit.backend import QoqoQiskitBackend

from typing import List, Any


def test_constructor():
    simulator = AerSimulator()
    wrong_simulator = AerSimulator.from_backend(FakeAthens())
    try:
        _ = QoqoQiskitBackend()
        _ = QoqoQiskitBackend(simulator)
    except:
        assert False

    with pytest.raises(TypeError) as exc:
        _ = QoqoQiskitBackend("wrong_name")
        assert "The input is not a valid Qiskit Backend instance." in str(exc.value)

    with pytest.raises(ValueError) as exc:
        _ = QoqoQiskitBackend(wrong_simulator)
        assert "Input a simulator from the following allowed list: {ALLOWED_PROVIDERS}" in str(
            exc.value)


@pytest.mark.parametrize("operations", [
    [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
    [ops.Hadamard(0), ops.CNOT(0, 1), ops.CNOT(1, 2), ops.CNOT(2, 3), ops.CNOT(3, 4)],
    [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)]
])
def test_run_circuit(operations: List[Any]):
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    with pytest.raises(ValueError) as exc:
        _ = backend.run_circuit(circuit)
        assert "The Circuit does not contain Measurement operations. Simulation not possible." in str(
            exc.value)

    circuit += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit += ops.PragmaRepeatedMeasurement("ri", 10)

    try:
        _ = backend.run_circuit(circuit)
    except:
        assert False, f"Correct Circuit failed on '.run_circuit()' call."


@pytest.mark.parametrize("operations", [
    [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
    [ops.Hadamard(0), ops.CNOT(0, 1), ops.CNOT(1, 2), ops.CNOT(2, 3), ops.CNOT(3, 4)],
    [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)]
])
def test_measurement_register_classicalregister(operations: List[Any]):
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    circuit += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit += ops.PragmaRepeatedMeasurement("ri", 10)

    measurement = ClassicalRegister(constant_circuit=None, circuits=[circuit])

    try:
        output = backend.run_measurement_registers(measurement=measurement)
    except:
        assert False

    assert output[0]["ri"]
    assert len(output[0]["ri"][0]) == len(involved_qubits)
    assert not output[1]
    assert not output[2]


@pytest.mark.parametrize("operations", [
    [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
    [ops.Hadamard(0), ops.CNOT(0, 1), ops.CNOT(1, 2), ops.CNOT(2, 3), ops.CNOT(3, 4)],
    [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)]
])
def test_measurement(operations: List[Any]):
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    circuit += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit += ops.PragmaRepeatedMeasurement("ri", 10)

    input = PauliZProductInput(number_qubits=len(involved_qubits), use_flipped_measurement=True)

    measurement = PauliZProduct(constant_circuit=None, circuits=[circuit], input=input)

    try:
        _ = backend.run_measurement(measurement=measurement)
    except:
        assert False


def test_run_options():
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    circuit += ops.Hadamard(0)
    circuit += ops.CNOT(0, 1)

    circuit += ops.DefinitionBit("ri", 2, True)
    circuit += ops.MeasureQubit(0, "ri", 0)
    circuit += ops.MeasureQubit(1, "ri", 1)
    circuit += ops.PragmaRepeatedMeasurement("ri", 10)

    with pytest.raises(ValueError) as exc:
        _ = backend.run_circuit(circuit)
        assert "Only input Circuits containing one type of measurement." in str(
            exc.value)

# For pytest
if __name__ == '__main__':
    pytest.main(sys.argv)
