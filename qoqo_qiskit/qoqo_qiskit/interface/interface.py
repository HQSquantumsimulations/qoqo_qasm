# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Qiskit interface for qoqo circuits."""

from qoqo import Circuit
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister

from qoqo_qasm import QasmBackend

from typing import Tuple, Optional


def to_qiskit_circuit(
    circuit: Circuit, qubit_register_name: Optional[str] = None
) -> Tuple[QuantumCircuit, dict[str, int]]:
    """Applies the qoqo Circuit -> Qiskit QuantumCircuit conversion.

    Args:
        circuit (Circuit): the qoqo Circuit to port.
        qubit_register_name (Optional[str]): the name of the qubit register.

    Returns:
        Tuple[QuantumCircuit, dict[str, int]]: the equivalent QuantumCircuit and the dict containing
                                     info for Qiskit's simulator.
    """
    # Populating dict output. Currently handling:
    #   - PragmaSetStateVector (continues further down)
    #   - PragmaSetNumberOfMeasurement
    #   - PragmaRepeatedMeasurement
    #   - MeasureQubit
    filtered_circuit = Circuit()
    sim_dict = {}
    sim_dict["MeasurementInfo"] = {}
    sim_dict["SimulationInfo"] = {}
    sim_dict["SimulationInfo"]["PragmaGetStateVector"] = False
    sim_dict["SimulationInfo"]["PragmaGetDensityMatrix"] = False
    initial_statevector = []
    for op in circuit:
        if "PragmaSetStateVector" in op.tags():
            initial_statevector = op.statevector()
        elif "PragmaSetNumberOfMeasurements" in op.tags():
            if "PragmaSetNumberOfMeasurements" not in sim_dict["MeasurementInfo"]:
                sim_dict["MeasurementInfo"]["PragmaSetNumberOfMeasurements"] = []
            sim_dict["MeasurementInfo"]["PragmaSetNumberOfMeasurements"].append(
                (op.readout(), op.number_measurements())
            )
        elif "PragmaRepeatedMeasurement" in op.tags():
            if "PragmaRepeatedMeasurement" not in sim_dict["MeasurementInfo"]:
                sim_dict["MeasurementInfo"]["PragmaRepeatedMeasurement"] = []
            sim_dict["MeasurementInfo"]["PragmaRepeatedMeasurement"].append(
                (op.readout(), op.number_measurements(), op.qubit_mapping())
            )
            filtered_circuit += op
        elif "MeasureQubit" in op.tags():
            if "MeasureQubit" not in sim_dict["MeasurementInfo"]:
                sim_dict["MeasurementInfo"]["MeasureQubit"] = []
            sim_dict["MeasurementInfo"]["MeasureQubit"].append(
                (op.qubit(), op.readout(), op.readout_index())
            )
            filtered_circuit += op
        elif "PragmaGetStateVector" in op.tags():
            sim_dict["SimulationInfo"]["PragmaGetStateVector"] = True
        elif "PragmaGetDensityMatrix" in op.tags():
            sim_dict["SimulationInfo"]["PragmaGetDensityMatrix"] = True
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

    return (return_circuit, sim_dict)
