# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit circuit conversion functions."""

from qoqo import Circuit
from qiskit import QuantumCircuit

from typing import Tuple


def to_qiskit_circuit(circuit: Circuit) -> Tuple[QuantumCircuit, dict]:
    """Applies the qoqo Circuit -> Qiskit QuantumCircuit conversion.

    Args:
        circuit (Circuit): the qoqo Circuit to port.

    Returns:
        Tuple[QuantumCircuit, dict]: the equivalent QuantumCircuit and a dict containing
                                     info for Qiskit's simulator.
    """
    return ()
