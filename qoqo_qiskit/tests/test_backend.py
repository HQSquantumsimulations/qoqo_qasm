# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for backend.py."""

import pytest
import sys

from qiskit import Aer

from qoqo_qiskit.backend import QoqoQiskitSimulator


def test_constructor():
    simulator = Aer.get_backend("aer_simulator")
    wrong_simulator = Aer.get_backend("qasm_simulator")
    try:
        _ = QoqoQiskitSimulator()
        _ = QoqoQiskitSimulator(simulator)
    except ValueError:
        assert False

    with pytest.raises(TypeError) as exc:
        _ = QoqoQiskitSimulator("wrong_name")
        assert "The input is not a valid Qiskit Backend instance." in str(exc.value)

    with pytest.raises(ValueError) as exc:
        _ = QoqoQiskitSimulator(wrong_simulator)
        assert "Input a simulator from the following allowed list: {ALLOWED_PROVIDERS}" in str(exc.value)

def test_run_circuit():
    pass

def test_measurement_register():
    pass

def test_measurement():
    pass

# For pytest
if __name__ == '__main__':
    pytest.main(sys.argv)