# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit backend for simulation purposes."""

from qoqo import QuantumProgram, Circuit
from qiskit import Aer
from qiskit.providers import Backend

from qoqo_qiskit.interface import to_qiskit_circuit  # type:ignore

from typing import Tuple, Union, Dict, List, cast

ALLOWED_PROVIDERS = ["aer_simulator"]


class QoqoQiskitSimulator:
    """Simulate a Qoqo QuantumProgram on a Qiskit simulator."""

    def __init__(self, simulator: Backend = None) -> None:
        """Init for Qiskit simulator settings.

        Args:
            simulator (Backend): Qiskit backend instance to use for the simulation.

        Raises:
            ValueError: the selected provider is not allowed.
        """
        if simulator is None:
            self.simulator = Aer.get_backend("aer_simulator")
        elif simulator.name() not in ALLOWED_PROVIDERS:
            raise ValueError(
                f"Input a simulator from the following allowed list: {ALLOWED_PROVIDERS}")

    def run_circuit(self, circuit: Circuit) -> Tuple[Dict[str, List[List[bool]]],
                                                     Dict[str, List[List[float]]],
                                                     Dict[str, List[List[complex]]]]:
        """Simulate a Circuit on a Qiskit simulator.

        Args:
            circuit (Circuit): the Circuit to simulate.

        Returns:
            Tuple[Dict[str, List[List[bool]]],
                  Dict[str, List[List[float]]],
                  Dict[str, List[List[complex]]]]: bit, float and complex registers dictionaries.

        Raises:
            ValueError: the Circuit does not contain Measurement operations
        """
        # Initializing the classical registers for calculation and output
        internal_bit_register_dict: Dict[str, List[bool]] = dict()
        internal_float_register_dict: Dict[str, List[float]] = dict()
        internal_complex_register_dict: Dict[str, List[complex]] = dict()

        output_bit_register_dict: Dict[str, List[List[bool]]] = dict()
        output_float_register_dict: Dict[str, List[List[float]]] = dict()
        output_complex_register_dict: Dict[str, List[List[complex]]] = dict()

        for bit_def in circuit.filter_by_tag("DefinitionBit"):
            internal_bit_register_dict[bit_def.name()] = [False for _ in range(bit_def.length())]
            if bit_def.is_output():
                output_bit_register_dict[bit_def.name()] = list()

        for float_def in circuit.filter_by_tag("DefinitionFloat"):
            internal_float_register_dict[float_def.name()] = [
                0.0 for _ in range(float_def.length())]
            if float_def.is_output():
                output_float_register_dict[float_def.name()] = cast(List[List[float]], list())

        for complex_def in circuit.filter_by_tag("DefinitionComplex"):
            internal_complex_register_dict[complex_def.name()] = [
                complex(0.0) for _ in range(complex_def.length())]
            if complex_def.is_output():
                output_complex_register_dict[complex_def.name()] = cast(List[List[complex]], list())

        if not self._are_measurement_operations_in(circuit):
            raise ValueError(
                "The Circuit does not contain Measurement operations. Simulation not possible.")

        compiled_circuit, _ = to_qiskit_circuit(circuit)

        result = self.simulator.run(compiled_circuit).result()
        output_bit_register_dict, output_float_register_dict, output_complex_register_dict = \
            self._counts_to_registers(result.get_counts(compiled_circuit))

        return output_bit_register_dict, output_float_register_dict, output_complex_register_dict

    def run_measurement_registers():
        pass

    def run_measurement():
        pass

    def _counts_to_registers(
        self,
        counts: Union[Dict[str, int], List[Dict[str, int]]]
    ) -> Tuple[Dict[str, List[List[bool]]],
               Dict[str, List[List[float]]],
               Dict[str, List[List[complex]]]]:
        pass

    def _are_measurement_operations_in(self, input: Union[Circuit, QuantumProgram]) -> bool:
        # if isinstance(input, Circuit):
        for op in input:
            if "Measurement" in op.tags():
                return True
        return False
