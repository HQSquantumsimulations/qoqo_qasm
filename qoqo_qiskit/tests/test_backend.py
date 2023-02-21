# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for backend.py."""

import pytest
import sys

from qoqo_qiskit.backend import QoqoQiskitSimulator


def test_constructor():
    backend = QoqoQiskitSimulator()

def test_run_circuit():
    pass

def test_measurement_register():
    pass

def test_measurement():
    pass

# For pytest
if __name__ == '__main__':
    pytest.main(sys.argv)