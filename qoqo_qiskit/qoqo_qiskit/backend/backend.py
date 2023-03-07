# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qoqo-qiskit backend for simulation purposes."""

from qoqo import Circuit
from qiskit_aer import AerSimulator
from qiskit.providers import Backend

from qoqo_qiskit.interface import to_qiskit_circuit

from typing import Tuple, Union, Dict, List, cast, Any, Optional

ALLOWED_PROVIDERS = ["aer_simulator"]


class QoqoQiskitBackend:
    """Simulate a Qoqo QuantumProgram on a Qiskit simulator."""

    def __init__(self, simulator: Backend = None) -> None:
        """Init for Qiskit simulator settings.

        Args:
            simulator (Backend): Qiskit backend instance to use for the simulation.

        Raises:
            TypeError: the input is not a valid Qiskit Backend instance.
            ValueError: the selected simulator is not allowed.
        """
        if simulator is None:
            self.simulator = AerSimulator()
        elif not isinstance(simulator, Backend):
            raise TypeError("The input is not a valid Qiskit Backend instance.")
        elif simulator.name() not in ALLOWED_PROVIDERS:
            raise ValueError(
                f"Input a simulator from the following allowed list: {ALLOWED_PROVIDERS}"
            )

    def run_circuit(
        self, circuit: Circuit
    ) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
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
            internal_bit_register_dict[bit_def.name()] = [
                False for _ in range(bit_def.length())
            ]
            if bit_def.is_output():
                output_bit_register_dict[bit_def.name()] = list()

        for float_def in circuit.filter_by_tag("DefinitionFloat"):
            internal_float_register_dict[float_def.name()] = [
                0.0 for _ in range(float_def.length())
            ]
            if float_def.is_output():
                output_float_register_dict[float_def.name()] = cast(
                    List[List[float]], list()
                )

        for complex_def in circuit.filter_by_tag("DefinitionComplex"):
            internal_complex_register_dict[complex_def.name()] = [
                complex(0.0) for _ in range(complex_def.length())
            ]
            if complex_def.is_output():
                output_complex_register_dict[complex_def.name()] = cast(
                    List[List[complex]], list()
                )

        # Qiskit conversion
        compiled_circuit, run_options = to_qiskit_circuit(circuit)

        # Raise ValueError if no measurement is performed
        if not run_options["MeasurementInfo"]:
            raise ValueError(
                "The Circuit does not contain Measurement operations. Simulation not possible."
            )

        # Handle simulation Options
        # TODO
        # TODO warning with multiple measurements

        # Simulation
        result = self.simulator.run(compiled_circuit, shots=1, memory=True).result()

        # Result transformation
        transformed_counts = self._counts_to_registers(result.get_memory())
        for reg in output_bit_register_dict:
            output_bit_register_dict[reg] = transformed_counts.pop()

        return (
            output_bit_register_dict,
            output_float_register_dict,
            output_complex_register_dict,
        )

    def run_measurement_registers(
        self, measurement: Any
    ) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
        """Run all circuits of a measurement with the PyQuEST backend.

        Args:
            measurement: The measurement that is run.

        Returns:
            Tuple[Dict[str, List[List[bool]]],
                  Dict[str, List[List[float]]],
                  Dict[str, List[List[complex]]]]
        """
        constant_circuit = measurement.constant_circuit()
        output_bit_register_dict: Dict[str, List[List[bool]]] = dict()
        output_float_register_dict: Dict[str, List[List[float]]] = dict()
        output_complex_register_dict: Dict[str, List[List[complex]]] = dict()

        for circuit in measurement.circuits():
            if constant_circuit is None:
                run_circuit = circuit
            else:
                run_circuit = constant_circuit + circuit

            (
                tmp_bit_register_dict,
                tmp_float_register_dict,
                tmp_complex_register_dict,
            ) = self.run_circuit(run_circuit)

            output_bit_register_dict.update(tmp_bit_register_dict)
            output_float_register_dict.update(tmp_float_register_dict)
            output_complex_register_dict.update(tmp_complex_register_dict)

        return (
            output_bit_register_dict,
            output_float_register_dict,
            output_complex_register_dict,
        )

    def run_measurement(self, measurement: Any) -> Optional[Dict[str, float]]:
        """Run a circuit with the PyQuEST backend.

        Args:
            measurement: The measurement that is run.

        Returns:
            Optional[Dict[str, float]]
        """
        (
            output_bit_register_dict,
            output_float_register_dict,
            output_complex_register_dict,
        ) = self.run_measurement_registers(measurement)

        return measurement.evaluate(
            output_bit_register_dict,
            output_float_register_dict,
            output_complex_register_dict,
        )

    def _counts_to_registers(
        self, counts: List[str]  # result.get_memory() style
    ) -> Union[List[bool], List[List[bool]]]:
        bit_map = []
        reg_num = counts[0].count(" ")
        for _ in range(reg_num + 1):
            bit_map.append([])
        for count in counts:
            splitted = count.split()
            for id, measurement in enumerate(splitted):
                measurement = self._bit_to_bool(measurement)
                bit_map[id].append(measurement)
        return bit_map

    def _are_measurement_operations_in(self, input: Circuit) -> bool:
        for op in input:
            if "Measurement" in op.tags():
                return True
        return False

    def _bit_to_bool(self, element: str) -> Union[bool, List[bool]]:
        ret = []
        for char in element:
            ret.append(char.lower() in ("1"))
        return ret
