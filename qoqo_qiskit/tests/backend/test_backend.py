# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for backend.py."""

import pytest
import sys

from qoqo import Circuit
from qoqo import operations as ops
from qoqo.measurements import (  # type:ignore
    ClassicalRegister,
    PauliZProduct,
    PauliZProductInput,
)
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
    assert "Input a Backend from the following allowed list: " in str(exc.value)


@pytest.mark.parametrize(
    "operations",
    [
        [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
        [
            ops.Hadamard(0),
            ops.CNOT(0, 1),
            ops.CNOT(1, 2),
            ops.CNOT(2, 3),
            ops.CNOT(3, 4),
        ],
        [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)],
    ],
)
def test_run_circuit_errors(operations: List[Any]):
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    with pytest.raises(ValueError) as exc:
        _ = backend.run_circuit(circuit)
    assert (
        "The Circuit does not contain Measurement, PragmaGetStateVector or PragmaGetDensityMatrix operations. Simulation not possible."
        in str(exc.value)
    )

    circuit_1 = Circuit()
    circuit_1 += circuit
    circuit_1 += ops.DefinitionComplex("ri", len(involved_qubits), True)
    circuit_1 += ops.PragmaGetStateVector("ri", None)
    circuit_1 += ops.PragmaGetDensityMatrix("ri", None)

    with pytest.raises(ValueError) as exc:
        _ = backend.run_circuit(circuit_1)
    assert (
        "The Circuit contains both a PragmaGetStateVector and a PragmaGetDensityMatrix instruction. Simulation not possible."
        in str(exc.value)
    )

    circuit_2 = Circuit()
    circuit_2 += circuit
    circuit_2 += ops.DefinitionBit("ri", len(involved_qubits), True)
    for i in range(len(involved_qubits)):
        circuit_2 += ops.MeasureQubit(i, "ri", i)
    circuit_2 += ops.PragmaRepeatedMeasurement("ri", 10)

    with pytest.raises(ValueError) as exc:
        _ = backend.run_circuit(circuit_2)
    assert "Only input Circuits containing one type of measurement." in str(exc.value)

    circuit_3 = Circuit()
    circuit_3 += circuit
    circuit_3 += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit_3 += ops.PragmaRepeatedMeasurement("ri", 10)

    try:
        _ = backend.run_circuit(circuit_3)
    except:
        assert False, f"Correct Circuit failed on '.run_circuit()' call."


@pytest.mark.parametrize(
    "operations",
    [
        [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
        [
            ops.Hadamard(0),
            ops.CNOT(0, 1),
            ops.CNOT(1, 2),
            ops.CNOT(2, 3),
            ops.CNOT(3, 4),
        ],
        [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)],
    ],
)
def test_run_circuit_results(operations: List[Any]):
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    circuit_1 = Circuit()
    circuit_1 += circuit
    circuit_1 += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit_1 += ops.PragmaRepeatedMeasurement("ri", 10)

    result = backend.run_circuit(circuit_1)

    assert result[0]
    assert result[0]["ri"]
    assert not result[1]
    assert not result[2]

    circuit_2 = Circuit()
    circuit_2 += circuit
    circuit_2 += ops.DefinitionComplex("ri", len(involved_qubits), True)
    circuit_2 += ops.PragmaGetStateVector("ri", None)

    result = backend.run_circuit(circuit_2)

    assert not result[0]
    assert not result[1]
    assert result[2]
    assert result[2]["ri"]
    assert len(result[2]["ri"][0]) == 2 ** len(involved_qubits)

    circuit_3 = Circuit()
    circuit_3 += circuit
    circuit_3 += ops.DefinitionComplex("ri", len(involved_qubits), True)
    circuit_3 += ops.PragmaGetDensityMatrix("ri", None)

    result = backend.run_circuit(circuit_3)

    assert not result[0]
    assert not result[1]
    assert result[2]
    assert result[2]["ri"]
    assert len(result[2]["ri"][0]) == (2 ** len(involved_qubits)) ** 2


@pytest.mark.parametrize(
    "operations",
    [
        [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
        [
            ops.Hadamard(0),
            ops.CNOT(0, 1),
            ops.CNOT(1, 2),
            ops.CNOT(2, 3),
            ops.CNOT(3, 4),
        ],
        [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)],
    ],
)
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


@pytest.mark.parametrize(
    "operations",
    [
        [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
        [
            ops.Hadamard(0),
            ops.CNOT(0, 1),
            ops.CNOT(1, 2),
            ops.CNOT(2, 3),
            ops.CNOT(3, 4),
        ],
        [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)],
    ],
)
def test_measurement(operations: List[Any]):
    backend = QoqoQiskitBackend()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    circuit += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit += ops.PragmaRepeatedMeasurement("ri", 10)

    input = PauliZProductInput(
        number_qubits=len(involved_qubits), use_flipped_measurement=True
    )

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

    circuit_0 = Circuit()
    circuit_0 += circuit
    circuit_0 += ops.DefinitionBit("ri", 2, True)
    circuit_0 += ops.PragmaRepeatedMeasurement("ri", 1000)

    result = backend.run_circuit(circuit_0)

    assert len(result[0]["ri"]) == 1000

    circuit_1 = Circuit()
    circuit_1 += circuit
    circuit_1 += ops.DefinitionBit("ro", 2, True)
    circuit_1 += ops.MeasureQubit(0, "ro", 0)
    circuit_1 += ops.MeasureQubit(1, "ro", 1)
    circuit_1 += ops.PragmaSetNumberOfMeasurements(250, "ro")

    result = backend.run_circuit(circuit_1)

    assert len(result[0]["ro"]) == 250


# For pytest
if __name__ == "__main__":
    pytest.main(sys.argv)
