# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit circuit conversion functions."""

from qoqo import Circuit
from qiskit import QuantumCircuit

from qoqo_qasm import QasmBackend

from typing import Tuple


def to_qiskit_circuit(circuit: Circuit) -> Tuple[QuantumCircuit, dict[str, int]]:
    """Applies the qoqo Circuit -> Qiskit QuantumCircuit conversion.

    Args:
        circuit (Circuit): the qoqo Circuit to port.

    Returns:
        Tuple[QuantumCircuit, dict[str, int]]: the equivalent QuantumCircuit and the dict containing
                                     info for Qiskit's simulator.
    """
    filtered_circuit = Circuit()
    sim_dict = {}
    sim_dict["measurements"] = []
    initial_statevector = []
    for op in circuit:
        if "PragmaSetStateVector" in op.tags():
            initial_statevector = op.statevector()
        elif "PragmaSetNumberOfMeasurements" in op.tags():
            sim_dict["measurements"].append((op.readout(), op.number_measurements(), {}))
            filtered_circuit += op
        elif "PragmaRepeatedMeasurement" in op.tags():
            sim_dict["measurements"].append(
                (op.readout(), op.number_measurements(), op.qubit_mapping()))
            filtered_circuit += op
        else:
            filtered_circuit += op

    qasm_backend = QasmBackend()
    input_qasm_str = qasm_backend.circuit_to_qasm_str(filtered_circuit)

    return_circuit = QuantumCircuit()
    from_qasm_circuit = QuantumCircuit().from_qasm_str(input_qasm_str)
    if len(initial_statevector) != 0:
        initial_circuit = QuantumCircuit(from_qasm_circuit.num_qubits, from_qasm_circuit.num_clbits)
        initial_circuit.initialize(initial_statevector)
        return_circuit = initial_circuit.compose(from_qasm_circuit)
    else:
        return_circuit = from_qasm_circuit

    return (return_circuit, sim_dict)
