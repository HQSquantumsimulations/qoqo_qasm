# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for backend.py."""

import pytest
import sys

from qoqo import Circuit
from qoqo import operations as ops

from qiskit import Aer

from qoqo_qiskit.backend import QoqoQiskitSimulator

from typing import List, Any


def test_constructor():
    simulator = Aer.get_backend("aer_simulator")
    wrong_simulator = Aer.get_backend("qasm_simulator")
    try:
        _ = QoqoQiskitSimulator()
        _ = QoqoQiskitSimulator(simulator)
    except:
        assert False

    with pytest.raises(TypeError) as exc:
        _ = QoqoQiskitSimulator("wrong_name")
        assert "The input is not a valid Qiskit Backend instance." in str(exc.value)

    with pytest.raises(ValueError) as exc:
        _ = QoqoQiskitSimulator(wrong_simulator)
        assert "Input a simulator from the following allowed list: {ALLOWED_PROVIDERS}" in str(exc.value)


@pytest.mark.parametrize("operations", [
    [ops.PauliX(1), ops.PauliX(0), ops.PauliZ(2), ops.PauliX(3), ops.PauliY(4)],
    [ops.Hadamard(0), ops.CNOT(0, 1), ops.CNOT(1, 2), ops.CNOT(2, 3), ops.CNOT(3, 4)],
    [ops.RotateX(0, 0.23), ops.RotateY(1, 0.12), ops.RotateZ(2, 0.34)]
])
def test_run_circuit(operations: List[Any]):
    backend = QoqoQiskitSimulator()

    circuit = Circuit()
    involved_qubits = set()
    for op in operations:
        involved_qubits.update(op.involved_qubits())
        circuit += op

    with pytest.raises(ValueError) as exc:
        _ = backend.run_circuit(circuit)
        assert "The Circuit does not contain Measurement operations. Simulation not possible." in str(exc.value)

    circuit += ops.DefinitionBit("ri", len(involved_qubits), True)
    circuit += ops.PragmaRepeatedMeasurement("ri", 10)

    try:
        _ = backend.run_circuit(circuit)
    except:
        assert False, f"Correct Circuit failed on '.run_circuit()' call."


def test_measurement_register():
    pass


def test_measurement():
    pass


# For pytest
if __name__ == '__main__':
    pytest.main(sys.argv)