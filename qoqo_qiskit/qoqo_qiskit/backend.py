# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit backend for simulation purposes."""

from qoqo import QuantumProgram, Circuit
from qiskit import Aer

from qoqo_qiskit.conversion import to_qiskit_circuit  # type:ignore

from typing import Tuple, Union

ALLOWED_PROVIDERS = ["aer_simulator"]


class QoqoQiskitSimulator:
    """Simulate a Qoqo QuantumProgram on a Qiskit simulator."""

    def __init__(self, simulator: str = "aer_simulator") -> None:
        """Init for Qiskit simulator settings.

        Args:
            simulator (str): String defining which Qiskit simulator to use.
                             Defaults to 'aer_simulator'.

        Raises:
            ValueError: the selected provider is not allowed.
        """
        if simulator not in ALLOWED_PROVIDERS:
            raise ValueError(  # TODO: flake8 says possible sql injection here
                "Select provider from the following allowed list: " + ', '.join(ALLOWED_PROVIDERS))

        self.simulator = simulator

    def simulate_circuit(self, circuit: Circuit) -> dict[str, int]:
        """Simulate a Circuit on a Qiskit simulator.

        Args:
            circuit (Circuit): the Circuit to simulate.

        Returns:
            dict[str, int]: dict containing, for each qubit, a str indicating its measurement count.

        Raises:
            ValueError: the Circuit does not contain Measurement operations
        """
        if not self._are_measurement_operations_in(circuit):
            raise ValueError(
                "The Circuit does not contain Measurement operations. Simulation not possible.")

        qiskit_circuit, _ = to_qiskit_circuit(circuit)

        simulator = Aer.get_backend(self.simulator)
        result = simulator.run(qiskit_circuit).result()
        counts = result.get_counts(qiskit_circuit)

        return counts

    #         TODO: restructure: build the circuit(s)
    # def simulate_quantum_program(self, quantum_program: QuantumProgram) -> list[dict[str, int]]:
    #     """Simulate a QuantumProgram on a Qiskit simulator.

    #     Args:
    #         quantum_program (QuantumProgram): the QuantumProgram to simulate.
    #     """
    #     if not self._are_measurement_operations_in(quantum_program):
    #         raise ValueError(
    #             "The QuantumProgram does not contain Measurement operations.
    # Simulation not possible.")

    #     constant_circuit_count = self.simulate_circuit(
    #         quantum_program.measurement.constant_circuit())
    #     simulated_circuits_counts = []
    #     for circ in quantum_program.measurement.circuits():
    #         circ_counts = self.simulate_circuit(circ)
    #         simulated_circuits_counts.append(circ_counts)

    def _counts_to_results() -> Tuple[str, int]:
        pass

    def _are_measurement_operations_in(self, input: Union[Circuit, QuantumProgram]) -> bool:
        # if isinstance(input, Circuit):
        for op in input:
            if "Measurement" in op.tags():
                return True
        return False
        # elif isinstance(input, QuantumProgram):
        #     for op in input.measurement.constant_circuit():
        #         if "Measurement" in op.tags():
        #             return True
        #     for circ in input.measurement.circuits():
        #         for op in circ:
        #             if "Measurement" in op.tags():
        #                 return True
        #     return False
