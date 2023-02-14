# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit backend for simulation purposes."""

from qoqo import QuantumProgram, Circuit
from qiskit import Aer

from qoqo_qiskit.conversion import to_qiskit_circuit  # type:ignore


class QoqoQiskitSimulator:
    """Simulate a Qoqo QuantumProgram on a Qiskit simulator."""

    def __init__(self, simulator: str = "aer_simulator") -> None:
        """Init for Qiskit simulator settings."""
        pass

    def simulate_circuit(self, circuit: Circuit) -> dict[str, int]:
        """Simulate a Circuit on a Qiskit simulator.

        Args:
            circuit (Circuit): the Circuit to simulate.

        Returns:
            dict[str, int]: dict containing, for each qubit, a str indicating its measurement count
        """
        qiskit_circuit = to_qiskit_circuit(circuit)
        simulator = Aer.get_backend("aer_simulator")
        result = simulator.run(qiskit_circuit).result()
        counts = result.get_counts(qiskit_circuit)

        return counts

    def simulate_quantum_program(self, quantum_program: QuantumProgram) -> list[dict[str, int]]:
        """Simulate a QuantumProgram on a Qiskit simulator.

        Args:
            quantum_program (QuantumProgram): the QuantumProgram to simulate.
        """
        constant_circuit_count = self.simulate_circuit(
            quantum_program.measurement.constant_circuit())
        simulated_circuits_counts = []
        for circ in quantum_program.measurement.circuits():
            circ_counts = self.simulate_circuit(circ)
            simulated_circuits_counts.append(circ_counts)

        
