# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for conversion.py."""

import pytest
import sys
from qoqo import Circuit
from qoqo import operations as ops
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister

from qoqo_qiskit.conversion import to_qiskit_circuit


def test_basic_circuit():
    circuit = Circuit()
    circuit += ops.Hadamard(0)
    circuit += ops.PauliX(1)

    qc = QuantumCircuit(2)
    qc.h(0)
    qc.x(1)

    out_circ, sim_dict = to_qiskit_circuit(circuit)

    assert (out_circ == qc)
    assert (len(sim_dict["measurements"]) == 0)


def test_qreg_creg_names():
    circuit = Circuit()
    circuit += ops.DefinitionBit('cr', 2, is_output=True)

    qr = QuantumRegister(1, 'qrg')
    cr = ClassicalRegister(2, 'cr')
    qc = QuantumCircuit(qr, cr)

    out_circ, _ = to_qiskit_circuit(circuit, qubit_register_name='qrg')

    assert (out_circ == qc)


def test_setstatevector():
    pass


def test_repeated_measurement():
    pass


def test_measure_qubit():
    pass


# For pytest
if __name__ == '__main__':
    pytest.main(sys.argv)
