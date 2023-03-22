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
"""Qiskit interface for qoqo circuits."""

from qoqo import Circuit
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister

from qoqo_qasm import QasmBackend

from typing import Tuple, Optional, Dict, Any


def to_qiskit_circuit(
    circuit: Circuit, qubit_register_name: Optional[str] = None
) -> Tuple[QuantumCircuit, Dict[str, int]]:
    """Applies the qoqo Circuit -> Qiskit QuantumCircuit conversion.

    Args:
        circuit (Circuit): the qoqo Circuit to port.
        qubit_register_name (Optional[str]): the name of the qubit register.

    Returns:
        Tuple[QuantumCircuit, Dict[str, int]]: the equivalent QuantumCircuit and the dict containing
                                     info for Qiskit's backend.
    """
    # Populating dict output. Currently handling:
    #   - PragmaSetStateVector (continues further down)
    #   - PragmaSetNumberOfMeasurement
    #   - PragmaRepeatedMeasurement
    #   - MeasureQubit
    filtered_circuit = Circuit()
    circuit_info: Dict[str, Any] = {}
    circuit_info["MeasurementInfo"] = {}
    circuit_info["SimulationInfo"] = {}
    circuit_info["SimulationInfo"]["PragmaGetStateVector"] = False
    circuit_info["SimulationInfo"]["PragmaGetDensityMatrix"] = False
    initial_statevector = []
    for op in circuit:
        if "PragmaSetStateVector" in op.tags():
            initial_statevector = op.statevector()
        elif "PragmaRepeatedMeasurement" in op.tags():
            if "PragmaRepeatedMeasurement" not in circuit_info["MeasurementInfo"]:
                circuit_info["MeasurementInfo"]["PragmaRepeatedMeasurement"] = []
            circuit_info["MeasurementInfo"]["PragmaRepeatedMeasurement"].append(
                (op.readout(), op.number_measurements(), op.qubit_mapping())
            )
            filtered_circuit += op
        elif "MeasureQubit" in op.tags():
            if "MeasureQubit" not in circuit_info["MeasurementInfo"]:
                circuit_info["MeasurementInfo"]["MeasureQubit"] = []
            circuit_info["MeasurementInfo"]["MeasureQubit"].append(
                (op.qubit(), op.readout(), op.readout_index())
            )
            filtered_circuit += op
        elif "PragmaSetNumberOfMeasurements" in op.tags():
            if "PragmaSetNumberOfMeasurements" not in circuit_info["SimulationInfo"]:
                circuit_info["SimulationInfo"]["PragmaSetNumberOfMeasurements"] = []
            circuit_info["SimulationInfo"]["PragmaSetNumberOfMeasurements"].append(
                (op.readout(), op.number_measurements())
            )
        elif "PragmaGetStateVector" in op.tags():
            circuit_info["SimulationInfo"]["PragmaGetStateVector"] = True
        elif "PragmaGetDensityMatrix" in op.tags():
            circuit_info["SimulationInfo"]["PragmaGetDensityMatrix"] = True
        else:
            filtered_circuit += op

    # qoqo_qasm call
    qasm_backend = QasmBackend(qubit_register_name=qubit_register_name)
    input_qasm_str = qasm_backend.circuit_to_qasm_str(filtered_circuit)

    # Handling PragmaSetStateVector + QASM -> Qiskit transformation
    return_circuit = QuantumCircuit()
    from_qasm_circuit = QuantumCircuit.from_qasm_str(input_qasm_str)
    if len(initial_statevector) != 0:
        qregs = []
        for qreg in from_qasm_circuit.qregs:
            qregs.append(QuantumRegister(qreg.size, qreg.name))
        cregs = []
        for creg in from_qasm_circuit.cregs:
            cregs.append(ClassicalRegister(creg.size, creg.name))
        regs = qregs + cregs
        initial_circuit = QuantumCircuit(*regs)
        initial_circuit.initialize(initial_statevector)
        return_circuit = initial_circuit.compose(from_qasm_circuit)
    else:
        return_circuit = from_qasm_circuit

    return (return_circuit, circuit_info)
