# Copyright © 2023 HQS Quantum Simulations GmbH.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
"""Qoqo-qiskit backend for simulation purposes."""

import numpy as np
from qoqo import Circuit
from qiskit_aer import AerSimulator
from qiskit.providers import Backend
from qiskit import QuantumCircuit, execute

from qoqo_qiskit.interface import to_qiskit_circuit

from typing import Tuple, Dict, List, cast, Any, Optional


class QoqoQiskitBackend:
    """Simulate a Qoqo QuantumProgram on a Qiskit backend."""

    def __init__(self, qiskit_backend: Backend = None, memory: bool = False) -> None:
        """Init for Qiskit backend settings.

        Args:
            qiskit_backend (Backend): Qiskit backend instance to use for the simulation.
            memory (bool): Whether the output will return the actual single shots instead
                           of an equivalent sequence taken from a result summary.

        Raises:
            TypeError: the input is not a valid Qiskit Backend instance.
        """
        if qiskit_backend is None:
            self.qiskit_backend = AerSimulator()
        elif not isinstance(qiskit_backend, Backend):
            raise TypeError("The input is not a valid Qiskit Backend instance.")
        else:
            self.qiskit_backend = qiskit_backend
        self.memory = memory

    def run_circuit(
        self, circuit: Circuit
    ) -> Tuple[
        Dict[str, List[List[bool]]],
        Dict[str, List[List[float]]],
        Dict[str, List[List[complex]]],
    ]:
        """Simulate a Circuit on a Qiskit backend.

        The default number of shots for the simulation is 200.
        Any kind of Measurement, Statevector or DensityMatrix instruction only works as intended if
        they are the last instructions in the Circuit.
        Currently only one simulation is performed, meaning different measurements on different
        registers are not supported.

        Args:
            circuit (Circuit): the Circuit to simulate.

        Returns:
            Tuple[Dict[str, List[List[bool]]],
                  Dict[str, List[List[float]]],
                  Dict[str, List[List[complex]]]]: bit, float and complex registers dictionaries.

        Raises:
            ValueError: the Circuit does not contain Measurement operations
        """
        clas_regs_sizes: Dict[str, int] = dict()

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
            clas_regs_sizes[bit_def.name()] = bit_def.length()
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
        res = to_qiskit_circuit(circuit)
        compiled_circuit: QuantumCircuit = res[0]
        run_options: Dict[str, Any] = res[1]

        # Raise ValueError:
        #   - if no measurement of any kind and no Pragmas are involved
        if (
            not run_options["MeasurementInfo"]
            and not run_options["SimulationInfo"]["PragmaGetStateVector"]
            and not run_options["SimulationInfo"]["PragmaGetDensityMatrix"]
        ):
            raise ValueError(
                "The Circuit does not contain Measurement, PragmaGetStateVector"
                " or PragmaGetDensityMatrix operations. Simulation not possible."
            )
        #   - if both StateVector and DensityMatrix pragmas are involved
        if (
            run_options["SimulationInfo"]["PragmaGetStateVector"]
            and run_options["SimulationInfo"]["PragmaGetDensityMatrix"]
        ):
            raise ValueError(
                "The Circuit contains both a PragmaGetStateVector"
                " and a PragmaGetDensityMatrix instruction. Simulation not possible."
            )
        #   - if more than 1 type of measurement is involved
        if len(run_options["MeasurementInfo"]) > 1:
            raise ValueError("Only input Circuits containing one type of measurement.")

        # Handle simulation Options
        shots = 200
        custom_shots = 0
        sim_type = "automatic"
        if run_options["SimulationInfo"]["PragmaGetStateVector"]:
            compiled_circuit.save_statevector()
            sim_type = "statevector"
        elif run_options["SimulationInfo"]["PragmaGetDensityMatrix"]:
            compiled_circuit.save_density_matrix()
            sim_type = "density_matrix"
        if "PragmaRepeatedMeasurement" in run_options["MeasurementInfo"]:
            for el in run_options["MeasurementInfo"]["PragmaRepeatedMeasurement"]:
                if el[1] > custom_shots:
                    custom_shots = el[1]
        if "PragmaSetNumberOfMeasurements" in run_options["SimulationInfo"]:
            for el in run_options["SimulationInfo"]["PragmaSetNumberOfMeasurements"]:
                if el[1] > custom_shots:
                    custom_shots = el[1]
        if custom_shots != 0:
            shots = custom_shots

        # Simulation
        result = execute(
            compiled_circuit, self.qiskit_backend, shots=shots, memory=self.memory
        ).result()

        # Result transformation
        if sim_type == "automatic":
            if self.memory:
                transformed_counts = self._counts_to_registers(
                    result.get_memory(), True, clas_regs_sizes
                )
            else:
                transformed_counts = self._counts_to_registers(
                    result.get_counts(), False, clas_regs_sizes
                )
            for id, reg in enumerate(output_bit_register_dict):
                reversed_list = []
                for shot in transformed_counts[id]:
                    reversed_list.append(shot[::-1])
                output_bit_register_dict[reg] = reversed_list
        elif sim_type == "statevector":
            vector = list(np.asarray(result.data(0)["statevector"]).flatten())
            for reg in output_complex_register_dict:
                output_complex_register_dict[reg].append(vector)
        elif sim_type == "density_matrix":
            vector = list(np.asarray(result.data(0)["density_matrix"]).flatten())
            for reg in output_complex_register_dict:
                output_complex_register_dict[reg].append(vector)

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
        """Run all circuits of a measurement with the Qiskit backend.

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
        """Run a circuit with the Qiskit backend.

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
        self, counts: Any, mem: bool, clas_regs_sizes: Dict[str, int]
    ) -> List[List[List[bool]]]:
        bit_map: List[List[List[bool]]] = []
        reg_num = 0
        for key in clas_regs_sizes:
            reg_num += clas_regs_sizes[key]
        for _ in range(reg_num):
            bit_map.append([])
        for key in counts:
            splitted = self._split(key, clas_regs_sizes)
            for id, measurement in enumerate(splitted):
                transf_measurement = self._bit_to_bool(measurement)
                if mem:
                    bit_map[id].append(transf_measurement)
                else:
                    for _ in range(counts[key]):
                        bit_map[id].append(transf_measurement)
        return bit_map

    def _are_measurement_operations_in(self, input: Circuit) -> bool:
        for op in input:
            if "Measurement" in op.tags():
                return True
        return False

    def _bit_to_bool(self, element: str) -> List[bool]:
        ret = []
        for char in element:
            ret.append(char.lower() in ("1"))
        return ret

    def _split(self, element: str, clas_regs_sizes: Dict[str, int]) -> List[str]:
        splitted: list[str] = []
        if " " in element:
            splitted = element.split()
            splitted.reverse()
        else:
            element = element[::-1]
            for key in clas_regs_sizes:
                splitted.append(element[: clas_regs_sizes[key] :])
                splitted[-1] = splitted[-1][::-1]
                element = element[clas_regs_sizes[key] :]
        return splitted
