# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit circuit conversion functions."""

from qoqo import Circuit
from qiskit import QuantumCircuit

from qoqo_qasm import QasmBackend

from typing import Tuple


def to_qiskit_circuit(circuit: Circuit) -> Tuple[QuantumCircuit, dict]:
    """Applies the qoqo Circuit -> Qiskit QuantumCircuit conversion.

    Args:
        circuit (Circuit): the qoqo Circuit to port.

    Returns:
        Tuple[QuantumCircuit, dict]: the equivalent QuantumCircuit and the dict containing
                                     info for Qiskit's simulator.
    """
    qasm_backend = QasmBackend()
    input_qasm_str = qasm_backend.circuit_to_qasm_str(circuit)
    return_circuit = QuantumCircuit().from_qasm_str(input_qasm_str)

    sim_dict = {}
    sim_dict["measurements"] = []
    for op in circuit:  # TODO: PragmaSetStateVector
        if "PragmaSetNumberOfMeasurements" in op.tags():
            sim_dict["measurements"].append((op.readout(), op.number_measurements(), {}))
        if "PragmaRepeatedMeasurement" in op.tags():
            sim_dict["measurements"].append(
                (op.readout(), op.number_measurements(), op.qubit_mapping()))

    return (return_circuit, sim_dict)
