# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit backend for simulation purposes."""

from qoqo import QuantumProgram, Circuit
from qiskit import Aer, Backend

from qoqo_qiskit.conversion import to_qiskit_circuit  # type:ignore

from typing import Tuple, Union

ALLOWED_PROVIDERS = ["aer_simulator"]
error_str = f"Select provider from the following allowed list: {ALLOWED_PROVIDERS}"  


class QoqoQiskitSimulator:
    """Simulate a Qoqo QuantumProgram on a Qiskit simulator."""

    def __init__(self, simulator: Backend = None) -> None:
        """Init for Qiskit simulator settings.

        Args:
            simulator (str): String defining which Qiskit simulator to use.
                             Defaults to 'aer_simulator'.

        Raises:
            ValueError: the selected provider is not allowed.
        """
        if simulator not in ALLOWED_PROVIDERS:
            raise ValueError(error_str)

        self.simulator = simulator  # create Aer

    def run_circuit(self, circuit: Circuit) -> dict[str, int]:
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

        result = self.simulator.run(qiskit_circuit).result()
        counts = result.get_counts(qiskit_circuit)

        return counts

    #         TODO: restructure: build the circuit(s)
    def run_quantum_program(self, quantum_program: QuantumProgram) -> list[dict[str, int]]:
        """Simulate a QuantumProgram on a Qiskit simulator.

        Args:
            quantum_program (QuantumProgram): the QuantumProgram to simulate.
        """
        if not self._are_measurement_operations_in(quantum_program):
            raise ValueError(
                "The QuantumProgram does not contain Measurement operations. Simulation not possible.")

        quantum_program.run(self.simulator, )

    def run_measurement_registers():
        pass

    def run_measurement_registers():
        pass

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
