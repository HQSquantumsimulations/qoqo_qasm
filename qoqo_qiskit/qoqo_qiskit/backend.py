# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit backend for simulation purposes."""

from qoqo import QuantumProgram, Circuit
# from qiskit import QuantumCircuit

from qoqo_qiskit.conversion import to_qiskit_circuit  # type:ignore


class QoqoQiskitSimulator:
    """Simulate a Qoqo QuantumProgram on a Qiskit simulator."""

    def __init__(self) -> None:
        """Init for Qiskit simulator settings."""
        pass

    def simulate_circuit(self, circuit: Circuit):
        """Simulate a Circuit on a Qiskit simulator.

        Args:
            circuit (Circuit): the Circuit to simulate.
        """
        qiskit_circuit = to_qiskit_circuit(circuit)

    def simulate_quantum_program(self, quantum_program: QuantumProgram):
        """Simulate a QuantumProgram on a Qiskit simulator.

        Args:
            quantum_program (QuantumProgram): the QuantumProgram to simulate.
        """
        constant_circuit_trans = self.simulate_circuit(
            quantum_program.measurement.constant_circuit())
        transformed_circuits = []
        for circ in quantum_program.measurement.circuits():
            transf_circ = self.simulate_circuit(circ)
            transformed_circuits.append(transf_circ)
