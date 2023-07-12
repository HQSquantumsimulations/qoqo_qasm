// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
//
//! Testing the roqoqo-qasm Interface

use ndarray::array;
use qoqo_calculator::CalculatorFloat;
use roqoqo::operations::*;
use roqoqo::prelude::*;
use roqoqo::Circuit;
use roqoqo_qasm::{call_circuit, call_operation, gate_definition, Qasm3Dialect, QasmVersion};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::usize;
use test_case::test_case;

fn tmp_create_map() -> HashMap<usize, usize> {
    let mut hm = HashMap::new();
    hm.insert(0, 1);
    hm.insert(1, 0);
    hm
}

/// Test that all operations return the correct String: no dialect differences
#[test_case(Operation::from(PauliX::new(0)), "x q[0];"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "y q[0];"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)), "z q[0];"; "PauliZ")]
#[test_case(Operation::from(Hadamard::new(0)), "h q[0];"; "Hadamard")]
#[test_case(Operation::from(SGate::new(0)), "s q[0];"; "SGate")]
#[test_case(Operation::from(TGate::new(0)), "t q[0];"; "TGate")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(-PI))), "rx(-3.141592653589793) q[0];"; "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(-PI))), "ry(-3.141592653589793) q[0];"; "RotateY")]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(-PI))), "rz(-3.141592653589793) q[0];"; "RotateZ")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)), "sxdg q[0];"; "InvSqrtPauliX")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "cy q[0],q[1];"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), "cz q[0],q[1];"; "ControlledPauliZ")]
#[test_case(Operation::from(SWAP::new(0, 1)), "swap q[0],q[1];"; "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)), "iswap q[0],q[1];"; "ISwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)), "siswap q[0],q[1];"; "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)), "siswapdg q[0],q[1];"; "InvSqrtISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)), "fswap q[0],q[1];"; "FSwap")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(0.2), CalculatorFloat::from(0.2), CalculatorFloat::from(0.2))), "fsim(0.2,0.2,0.2) q[0],q[1];"; "Fsim")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))), "qsim(0.1,0.1,0.1) q[0],q[1];"; "Qsim")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from(0.2))), "pmint(0.2) q[0],q[1];"; "PMInteraction")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))), "gvnsrot(0.1,0.1) q[0],q[1];"; "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))), "gvnsrotle(0.1,0.1) q[0],q[1];"; "GivensRotationLittleEndian")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.1), CalculatorFloat::from(0.1))), "spinint(0.1,0.1,0.1) q[0],q[1];"; "SpinInteraction")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from(0.2))), "xy(0.2) q[0],q[1];"; "XY")]
#[test_case(Operation::from(RotateXY::new(0, CalculatorFloat::from(0.2), CalculatorFloat::from(0.2))), "rxy(0.2,0.2) q[0];"; "RotateXY")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(0.1))), "pscz(0.1) q[0],q[1];"; "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from(0.1), CalculatorFloat::from(0.2))), "pscp(0.1,0.2) q[0],q[1];"; "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(SingleQubitGate::new(0, CalculatorFloat::from(1.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0), CalculatorFloat::from(0.0))), "u3(0.000000000000000,0.000000000000000,-0.000000000000000) q[0];"; "SingleQubitGate")]
#[test_case(Operation::from(PragmaActiveReset::new(0)), "reset q[0];"; "PragmaActiveReset")]
#[test_case(Operation::from(MeasureQubit::new(0, "ro".to_string(), 0)), "measure q[0] -> ro[0];"; "MeasureQubit")]
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)), "ccz q[0],q[1],q[2];"; "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, 0.3.into())), "ccp(0.3) q[0],q[1],q[2];"; "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)), "ccx q[0],q[1],q[2];"; "Toffoli")]
fn test_call_operation_identical_2_3_all(operation: Operation, converted: &str) {
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0).unwrap(),
        converted.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        converted.to_string()
    );
    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        converted.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        converted.to_string()
    );
}

/// Test that all operations return the correct String: 2.0 vs. 3.0 differences
#[test_case(Operation::from(DefinitionFloat::new("ro".to_string(), 1, true)), "creg ro[1];", "output float[1] ro;"; "DefinitionFloat output")]
#[test_case(Operation::from(DefinitionFloat::new("ro".to_string(), 1, false)), "creg ro[1];", "float[1] ro;"; "DefinitionFloat")]
#[test_case(Operation::from(DefinitionUsize::new("ro".to_string(), 1, true)), "creg ro[1];", "output uint[1] ro;"; "DefinitionUsize ouput")]
#[test_case(Operation::from(DefinitionUsize::new("ro".to_string(), 1, false)), "creg ro[1];", "uint[1] ro;"; "DefinitionUsize")]
#[test_case(Operation::from(DefinitionBit::new("ro".to_string(), 1, true)), "creg ro[1];", "output bit[1] ro;"; "DefinitionBit output")]
#[test_case(Operation::from(DefinitionBit::new("ro".to_string(), 1, false)), "creg ro[1];", "bit[1] ro;"; "DefinitionBit")]
#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 1, true)), "creg ro[1];", "output float[1] ro_re;\noutput float[1] ro_im;"; "DefinitionComplex output")]
#[test_case(Operation::from(DefinitionComplex::new("ro".to_string(), 1, false)), "creg ro[1];", "float[1] ro_re;\nfloat[1] ro_im;"; "DefinitionComplex")]
#[test_case(Operation::from(InputSymbolic::new("other".to_string(), 0.0)), "", "input float other;"; "InputSymbolic")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(1.0))), "", "gphase 1e0;"; "PragmaGlobalPhase")]
fn test_call_operation_different_2_3(operation: Operation, converted_2: &str, converted_3: &str) {
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0).unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        converted_3.to_string()
    );
    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        converted_3.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        converted_3.to_string()
    );
}

/// Test that all operations return the correct String: 2.0 vs. 3.0 differences (Roqoqo dialect)
#[test_case(Operation::from(PragmaSleep::new(vec![0,1], CalculatorFloat::from(0.3))), "", "pragma roqoqo PragmaSleep [0, 1] 3e-1;"; "PragmaSleep")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0,1])), "", "pragma roqoqo PragmaStopDecompositionBlock [0, 1];"; "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![], CalculatorFloat::from(0.0))), "", "pragma roqoqo PragmaStopParallelBlock [] 0e0;"; "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(20, "ro".to_string())), "", "pragma roqoqo PragmaSetNumberOfMeasurements 20 ro;"; "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0,1], HashMap::new())), "", "pragma roqoqo PragmaStartDecompositionBlock [0, 1] {};"; "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(PragmaGetDensityMatrix::new("test".into(), None)), "", "pragma roqoqo PragmaGetDensityMatrix test ;"; "PragmaGetDensityMatrix")]
#[test_case(Operation::from(PragmaGetOccupationProbability::new("test".into(), None)), "", "pragma roqoqo PragmaGetOccupationProbability test ;"; "PragmaGetOccupationProbability")]
#[test_case(Operation::from(PragmaGetPauliProduct::new(HashMap::new(), "test".into(), Circuit::new())), "", "pragma roqoqo PragmaGetPauliProduct {} test ;"; "PragmaGetPauliProduct")]
#[test_case(Operation::from(PragmaGetStateVector::new("test".into(), None)), "", "pragma roqoqo PragmaGetStateVector test ;"; "PragmaGetStateVector")]
#[test_case(Operation::from(PragmaRepeatedMeasurement::new("ro".to_string(), 1, None)), "measure q -> ro;", "measure q -> ro;\npragma roqoqo PragmaSetNumberOfMeasurements 1 ro;"; "PragmaRepeatedMeasurement")]
fn test_call_operation_different_2_3_roqoqo_dialect(
    operation: Operation,
    converted_2: &str,
    converted_3: &str,
) {
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0).unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        converted_3.to_string()
    );
}

/// Test that all operations return the correct String: 2.0 vs. 3.0 differences (Braket dialect)
#[test_case(Operation::from(CNOT::new(0, 1)), "cx q[0],q[1];", "cnot q[0],q[1];"; "CNOT")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(PI/4.0))), "cp(7.853981633974483e-1) q[0],q[1];", "cphaseshift(7.853981633974483e-1) q[0],q[1];"; "ControlledPhaseShift")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), "rxx(pi/2) q[0],q[1];", "xx(pi/2) q[0],q[1];"; "MolmerSorensenXX")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from(PI/2.0))), "rxx(1.5707963267948966e0) q[0],q[1];", "xx(1.5707963267948966e0) q[0],q[1];"; "VariableMSXX")]
#[test_case(Operation::from(SqrtPauliX::new(0)), "sx q[0];", "v q[0];"; "SqrtPauliX")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI))), "p(3.141592653589793) q[0];", "phaseshift(3.141592653589793) q[0];"; "PhaseShiftState1")]
fn test_call_operation_different_2_3_braket_dialect(
    operation: Operation,
    converted_2: &str,
    converted_3: &str,
) {
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0).unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        converted_2.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        converted_3.to_string()
    );
}

/// Test that all operations return the correct error: 2.0 vs. 3.0 differences (Roqoqo dialect)
#[test_case(Operation::from(PragmaBoostNoise::new(1.5.into())), "pragma roqoqo PragmaBoostNoise 1.5e0;"; "PragmaBoostNoise")]
#[test_case(Operation::from(PragmaDamping::new(0, 1.0.into(), 1.5.into())), "pragma roqoqo PragmaDamping 0 1e0 1.5e0;"; "PragmaDamping")]
#[test_case(Operation::from(PragmaDephasing::new(0, 1.0.into(), 1.5.into())), "pragma roqoqo PragmaDephasing 0 1e0 1.5e0;"; "PragmaDephasing")]
#[test_case(Operation::from(PragmaDepolarising::new(0, 1.0.into(), 1.5.into())), "pragma roqoqo PragmaDepolarising 0 1e0 1.5e0;"; "PragmaDepolarising")]
#[test_case(Operation::from(PragmaGeneralNoise::new(0, 1.0.into(), array![[1.5]])), "pragma roqoqo PragmaGeneralNoise 0 1e0 [[1.5]];"; "PragmaGeneralNoise")]
#[test_case(Operation::from(PragmaOverrotation::new("Hadamard".into(), [0, 1].into(), 0.4, 0.5)), "pragma roqoqo PragmaOverrotation Hadamard [0, 1] 0.4 0.5;"; "PragmaOverrotation")]
#[test_case(Operation::from(PragmaRandomNoise::new(0, 0.4.into(), 0.5.into(), 0.3.into())), "pragma roqoqo PragmaRandomNoise 0 4e-1 5e-1 3e-1;"; "PragmaRandomNoise")]
#[test_case(Operation::from(PragmaRepeatGate::new(3)), "pragma roqoqo PragmaRepeatGate 3;"; "PragmaRepeatGate")]
#[test_case(Operation::from(PragmaSetDensityMatrix::new(array![[1.5.into()]])), "pragma roqoqo PragmaSetDensityMatrix [[1.5+0i]];"; "PragmaSetDensityMatrix")]
#[test_case(Operation::from(PragmaSetStateVector::new(array![1.5.into()])), "pragma roqoqo PragmaSetStateVector [1.5+0i];"; "PragmaSetStateVector")]
fn test_call_operation_error_2_3_roqoqo_dialect(operation: Operation, converted_3: &str) {
    let error = RoqoqoBackendError::OperationNotInBackend {
        backend: "QASM",
        hqslang: operation.hqslang(),
    };

    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap_err()
        .to_string(),
        error.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket))
            .unwrap_err()
            .to_string(),
        error.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0)
            .unwrap_err()
            .to_string(),
        error.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        converted_3.to_string()
    );
}

/// Test that all operations return the correct error: 2.0 vs. 3.0 differences (braket dialect)
#[test_case(Operation::from(GPi::new(0, CalculatorFloat::PI)), "gpi(3.141592653589793) q[0];"; "GPi")]
#[test_case(Operation::from(GPi2::new(0, CalculatorFloat::PI)), "gpi2(3.141592653589793) q[0];"; "GPi2")]
fn test_call_operation_error_2_3_braket_dialect(operation: Operation, converted_3: &str) {
    let error = RoqoqoBackendError::OperationNotInBackend {
        backend: "QASM",
        hqslang: operation.hqslang(),
    };

    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap_err()
        .to_string(),
        error.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo))
            .unwrap_err()
            .to_string(),
        error.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0)
            .unwrap_err()
            .to_string(),
        error.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        converted_3.to_string()
    );
}

/// Test that all operations return the correct error: 2.0 vs. 3.0 differences
#[test_case(Operation::from(InputBit::new("other".to_string(), 0, false)), "other[0] = false;"; "InputBit")]
fn test_call_operation_error_2_3(operation: Operation, converted_3: &str) {
    let error = RoqoqoBackendError::OperationNotInBackend {
        backend: "QASM",
        hqslang: operation.hqslang(),
    };

    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0),
        Err(error)
    );
    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        converted_3.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        converted_3.to_string()
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        converted_3.to_string()
    );
}

/// Test that all operations return the correct gate definition
#[test_case(Operation::from(PauliX::new(0)), "gate x a { u3(pi,0,pi) a; }"; "PauliX")]
#[test_case(Operation::from(PauliY::new(0)), "gate y a { u3(pi,pi/2,pi/2) a; }"; "PauliY")]
#[test_case(Operation::from(PauliZ::new(0)), "gate z a { u1(pi) a; }"; "PauliZ")]
#[test_case(Operation::from(Hadamard::new(0)), "gate h a { u2(0,pi) a; }"; "Hadamard")]
#[test_case(Operation::from(SGate::new(0)), "gate s a { u1(pi/2) a; }"; "SGate")]
#[test_case(Operation::from(TGate::new(0)), "gate t a { u1(pi/4) a; }"; "TGate")]
#[test_case(Operation::from(PhaseShiftState1::new(0, CalculatorFloat::from(PI))), "gate p(lambda) q { U(0,0,lambda) q; }"; "PhaseShiftState1")]
#[test_case(Operation::from(RotateX::new(0, CalculatorFloat::from(-PI))), "gate rx(theta) a { u3(theta,-pi/2,pi/2) a; }"; "RotateX")]
#[test_case(Operation::from(RotateY::new(0, CalculatorFloat::from(-PI))), "gate ry(theta) a { u3(theta,0,0) a; }"; "RotateY")]
#[test_case(Operation::from(RotateZ::new(0, CalculatorFloat::from(-PI))), "gate rz(phi) a { u1(phi) a; }"; "RotateZ")]
#[test_case(Operation::from(SqrtPauliX::new(0)), "gate sx a { u1(-pi/2) a; u2(0,pi) a; u1(-pi/2) a; }"; "SqrtPauliX")]
#[test_case(Operation::from(InvSqrtPauliX::new(0)), "gate sxdg a { u1(pi/2) a; u2(0,pi) a; u1(pi/2) a; }"; "InvSqrtPauliX")]
#[test_case(Operation::from(MolmerSorensenXX::new(0, 1)), "gate rxx(theta) a,b { u3(pi/2,theta,0) a; u2(0,pi) b; cx a,b; u1(-theta) b; cx a,b; u2(0,pi) b; u2(-pi,pi-theta) a; }"; "MolmerSorensenXX")]
#[test_case(Operation::from(CNOT::new(0, 1)), "gate cx c,t { CX c,t; }"; "CNOT")]
#[test_case(Operation::from(VariableMSXX::new(0, 1, CalculatorFloat::from(PI/2.0))), "gate rxx(theta) a,b { u3(pi/2,theta,0) a; u2(0,pi) b; cx a,b; u1(-theta) b; cx a,b; u2(0,pi) b; u2(-pi,pi-theta) a; }"; "VariableMSXX")]
#[test_case(Operation::from(ControlledPauliY::new(0, 1)), "gate cy a,b { u1(-pi/2) b; cx a,b; u1(pi/2) b; }"; "ControlledPauliY")]
#[test_case(Operation::from(ControlledPauliZ::new(0, 1)), "gate cz a,b { u2(0,pi) b; cx a,b; u2(0,pi) b; }"; "ControlledPauliZ")]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(PI/4.0))), "gate cp(lambda) a,b { U(0,0,lambda/2) a; cx a,b; U(0,0,-lambda/2) b; cx a,b; U(0,0,lambda/2) b; }"; "ControlledPhaseShift")]
#[test_case(Operation::from(SWAP::new(0, 1)), "gate swap a,b { cx a,b; cx b,a; cx a,b; }"; "SWAP")]
#[test_case(Operation::from(ISwap::new(0, 1)), "gate iswap a,b { rx(pi/2) a; cx a,b; rx(-pi/2) a; ry(-pi/2) b; cx a,b; rx(-pi/2) a; }"; "ISwap")]
#[test_case(Operation::from(SqrtISwap::new(0, 1)), "gate siswap a,b { rx(pi/2) a; cx a,b; rx(-pi/4) a; ry(-pi/4) b; cx a,b; rx(-pi/2) a; }"; "SqrtISwap")]
#[test_case(Operation::from(InvSqrtISwap::new(0, 1)), "gate siswapdg a,b { rx(pi/2) a; cx a,b; rx(pi/4) a; ry(pi/4) b; cx a,b; rx(-pi/2) a; }"; "InvSqrtISwap")]
#[test_case(Operation::from(FSwap::new(0, 1)), "gate fswap a,b { rz(-pi/2) a; rz(-pi/2) b; rx(pi/2) a; cx a,b; rx(-pi/2) a; ry(-pi/2) b; cx a,b; rx(-pi/2) a; }"; "FSwap")]
#[test_case(Operation::from(Fsim::new(0, 1, CalculatorFloat::from(PI/2.0), CalculatorFloat::from(PI/2.0), CalculatorFloat::from(PI/2.0))), "gate fsim(t,u,delta) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; ry(-t+delta+pi/2) a; rx(pi) a; ry(-pi/2) b; rz((u-pi)/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(pi) a; ry(t+delta+pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(-pi/2) b; rx(-pi/2) b; rz((-u-pi)/2) a; rz((-u-pi)/2) b; }"; "Fsim")]
#[test_case(Operation::from(PMInteraction::new(0, 1, CalculatorFloat::from(PI/2.0))), "gate pmint(theta) a,b { rx(pi/2) a; cx a,b; rx(theta) a; ry(theta) b; cx a,b; rx(-pi/2) a; }"; "PMInteraction")]
#[test_case(Operation::from(GivensRotation::new(0, 1, CalculatorFloat::from(0.2), CalculatorFloat::from(0.3))), "gate gvnsrot(theta,phi) a,b { rz(phi+pi/2) b; rx(pi/2) a; cx a,b; rx(-theta) a; ry(-theta) b; cx a,b; rx(-pi/2) a; rz(-pi/2) b; }"; "GivensRotation")]
#[test_case(Operation::from(GivensRotationLittleEndian::new(0, 1, CalculatorFloat::from(0.2), CalculatorFloat::from(0.3))), "gate gvnsrotle(theta,phi) a,b { rz(-pi/2) a; rx(pi/2) a; cx a,b; rx(-theta) a; ry(-theta) b; cx a,b; rx(-pi/2) a; rz(phi+pi/2) a; }"; "GivensRotationLittleEndian")]
#[test_case(Operation::from(Qsim::new(0, 1, CalculatorFloat::from(0.2), CalculatorFloat::from(0.3), CalculatorFloat::from(0.2))), "gate qsim(xc,yc,zc) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; ry(-2*xc+pi/2) a; rx(pi) a; ry(-pi/2) b; rz(2*zc-pi) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(pi) a; ry(2*yc+pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(-pi/2) b; rx(-pi/2) b; }"; "Qsim")]
#[test_case(Operation::from(XY::new(0, 1, CalculatorFloat::from(0.3))), "gate xy(theta) a,b { rx(pi/2) a; cx a,b; rx(-theta/2) a; ry(-theta/2) b; cx a,b; rx(-pi/2) a; }"; "XY")]
#[test_case(Operation::from(RotateXY::new(0, CalculatorFloat::from(0.3), CalculatorFloat::from(0.3))), "gate rxy(theta,phi) q { u3(theta,phi-pi/2,pi/2-phi) q; }"; "RotateXY")]
#[test_case(Operation::from(SpinInteraction::new(0, 1, CalculatorFloat::from(0.3), CalculatorFloat::from(0.3), CalculatorFloat::from(0.3))), "gate spinint(xc,yc,zc) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; ry(-2*xc) a; rx(pi) a; ry(-pi/2) b; rz(2*zc-pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(pi) a; ry(2*yc+pi) a; rz(pi) b; ry(pi/2) b; u2(0,pi) b; cx a,b; u2(0,pi) b; rz(-pi/2) b; rx(-pi/2) b; }"; "SpinInteraction")]
#[test_case(Operation::from(PhaseShiftedControlledZ::new(0, 1, CalculatorFloat::from(0.2))), "gate pscz(phi) a,b { rz(pi/2) a; rz(pi/2) b; ry(pi/2) b; cx a,b; rx(-pi/2) b; rz(-pi/2) a; ry(-pi/2) b; rz(phi) a; rz(phi) b; }"; "PhaseShiftedControlledZ")]
#[test_case(Operation::from(PhaseShiftedControlledPhase::new(0, 1, CalculatorFloat::from(0.2), CalculatorFloat::from(0.2))), "gate pscp(theta,phi) a,b { rz(theta/2) a; rz(theta/2) b; cx a,b; rz(-theta/2) b; cx a,b; rz(phi) a; rz(phi) b; }"; "PhaseShiftedControlledPhase")]
#[test_case(Operation::from(PragmaSleep::new(vec![0,1], CalculatorFloat::from(0.3))), ""; "PragmaSleep")]
#[test_case(Operation::from(PragmaGlobalPhase::new(CalculatorFloat::from(0.3))), ""; "PragmaGlobalPhase")]
#[test_case(Operation::from(PragmaStopDecompositionBlock::new(vec![0,1])), ""; "PragmaStopDecompositionBlock")]
#[test_case(Operation::from(PragmaStopParallelBlock::new(vec![], CalculatorFloat::from(0.0))), ""; "PragmaStopParallelBlock")]
#[test_case(Operation::from(PragmaSetNumberOfMeasurements::new(20, "ro".to_string())), ""; "PragmaSetNumberOfMeasurements")]
#[test_case(Operation::from(PragmaStartDecompositionBlock::new(vec![0,1], HashMap::new())), ""; "PragmaStartDecompositionBlock")]
#[test_case(Operation::from(InputSymbolic::new("other".to_string(), 0.0)), ""; "InputSymbolic")]
#[test_case(Operation::from(ControlledControlledPauliZ::new(0, 1, 2)), "gate ccz a,b,c { U(0,0,pi/4) b; cx b,c; U(0,0,-pi/4) c; cx b,c; U(0,0,pi/4) c; cx a,b; U(0,0,-pi/4) b; cx b,c; U(0,0,pi/4) c; cx b,c; U(0,0,-pi/4) c; cx a,b; U(0,0,pi/4) a; cx a,c; U(0,0,-pi/4) c; cx a,c; U(0,0,pi/4) c; }"; "ControlledControlledPauliZ")]
#[test_case(Operation::from(ControlledControlledPhaseShift::new(0, 1, 2, 0.3.into())), "gate ccp(theta) a,b,c { U(0,0,theta/4) b; cx b,c; U(0,0,-theta/4) c; cx b,c; U(0,0,theta/4) c; cx a,b; U(0,0,-theta/4) b; cx b,c; U(0,0,theta/4) c; cx b,c; U(0,0,-theta/4) c; cx a,b; U(0,0,theta/4) a; cx a,c; U(0,0,-theta/4) c; cx a,c; U(0,0,theta/4) c; }"; "ControlledControlledPhaseShift")]
#[test_case(Operation::from(Toffoli::new(0, 1, 2)), "gate ccx a,b,c { u2(0,pi) c; cx b,c; u1(-pi/4) c; cx a,c; u1(pi/4) c; cx b,c; u1(-pi/4) c; cx a,c; u1(pi/4) b; u1(pi/4) c; u2(0,pi) c; cx a,b; u1(pi/4) a; u1(-pi/4) b; cx a,b; }"; "Toffoli")]
fn test_gate_definition(operation: Operation, converted: &str) {
    assert_eq!(
        gate_definition(&operation, QasmVersion::V2point0).unwrap(),
        converted.to_string()
    )
}

/// Test that operations return the correct gate definition error
#[test_case(Operation::from(Bogoliubov::new(0, 1, 0.1.into(), 0.2.into())); "Bogoliubov")]
#[test_case(Operation::from(GPi::new(0, 0.1.into())); "GPi")]
#[test_case(Operation::from(GPi2::new(0, 0.2.into())); "GPi2")]
fn test_gate_definition_error(operation: Operation) {
    let error = RoqoqoBackendError::OperationNotInBackend {
        backend: "QASM",
        hqslang: operation.hqslang(),
    };
    assert_eq!(
        gate_definition(&operation, QasmVersion::V2point0),
        Err(error)
    )
}

/// Test PragmaConditional correct behaviour
#[test]
fn test_pragma_conditional() {
    let mut circuit = Circuit::new();
    circuit += Hadamard::new(0);
    circuit += PauliX::new(0);

    let mut break_circuit = circuit.clone();
    break_circuit += PragmaConditional::new("c".to_string(), 0, Circuit::new());
    let pcond = PragmaConditional::new("c".to_string(), 0, break_circuit);
    assert_eq!(
        call_operation(&Operation::from(pcond), "q", QasmVersion::V2point0),
        Err(RoqoqoBackendError::GenericError {
            msg: "For OpenQASM 2.0 we cannot have nested PragmaConditional operations".to_string()
        })
    );

    let pcond = PragmaConditional::new("c".to_string(), 0, circuit.clone());
    let data_2 = "if(c[0]==1) h q[0];\nif(c[0]==1) x q[0];";
    assert_eq!(
        call_operation(&Operation::from(pcond.clone()), "q", QasmVersion::V2point0).unwrap(),
        data_2
    );
    let data_3 = "if(c[0]==1) {\nh q[0];x q[0];}";
    assert_eq!(
        call_operation(
            &Operation::from(pcond.clone()),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Braket)
        )
        .unwrap(),
        data_3
    );
    assert_eq!(
        call_operation(
            &Operation::from(pcond.clone()),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        data_3
    );
    assert_eq!(
        call_operation(
            &Operation::from(pcond),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo)
        )
        .unwrap(),
        data_3
    );

    let mut break_circuit = Circuit::new();
    break_circuit += Bogoliubov::new(0, 1, 0.1.into(), 0.2.into());
    let pcond = PragmaConditional::new("c".to_string(), 0, break_circuit);
    let error = RoqoqoBackendError::OperationNotInBackend {
        backend: "QASM",
        hqslang: "Bogoliubov",
    };
    assert_eq!(
        call_operation(
            &Operation::from(pcond),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Braket)
        ),
        Err(error)
    );
}

/// Test PragmaLoop correct behaviour
#[test]
fn test_pragma_loop() {
    let mut circuit = Circuit::new();
    circuit += Hadamard::new(0);

    let pcond = PragmaLoop::new("test".into(), circuit.clone());

    assert_eq!(
        call_operation(&Operation::from(pcond.clone()), "q", QasmVersion::V2point0),
        Err(RoqoqoBackendError::GenericError {
            msg: "PragmaLoop not allowed with qasm_version 2.0".into()
        })
    );

    let data_3_roqoqo = "pragma roqoqo PragmaLoop test Hadamard(Hadamard { qubit: 0 })\n;";
    assert_eq!(
        call_operation(
            &Operation::from(pcond),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Roqoqo)
        )
        .unwrap(),
        data_3_roqoqo
    );

    let pcond = PragmaLoop::new("test".into(), circuit.clone());
    let qasm = QasmVersion::V3point0(Qasm3Dialect::Vanilla);
    assert_eq!(
        call_operation(&Operation::from(pcond.clone()), "q", qasm),
        Err(RoqoqoBackendError::GenericError { msg: format!("Used PragmaLoop with a string test for repetitions and a qasm-version that is incompatible: {qasm:?}") })
    );
    let qasm = QasmVersion::V3point0(Qasm3Dialect::Braket);
    assert_eq!(
        call_operation(&Operation::from(pcond), "q", qasm),
        Err(RoqoqoBackendError::GenericError { msg: format!("Used PragmaLoop with a string test for repetitions and a qasm-version that is incompatible: {qasm:?}") })
    );

    let pcond = PragmaLoop::new(2.0.into(), circuit);
    let data_3 = "for uint i in [0:2] {\n    h q[0];\n}";
    assert_eq!(
        call_operation(
            &Operation::from(pcond.clone()),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        )
        .unwrap(),
        data_3
    );
    assert_eq!(
        call_operation(
            &Operation::from(pcond),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Braket)
        )
        .unwrap(),
        data_3
    );

    let mut break_circuit = Circuit::new();
    break_circuit += Bogoliubov::new(0, 1, 0.1.into(), 0.2.into());
    let pcond = PragmaLoop::new(2.0.into(), break_circuit.clone());
    let error = RoqoqoBackendError::OperationNotInBackend {
        backend: "QASM",
        hqslang: "Bogoliubov",
    };
    assert_eq!(
        call_operation(
            &Operation::from(pcond),
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Braket)
        ),
        Err(error)
    );
}

/// Test PragmaRepeatedMeasurement correct behaviour
#[test]
fn test_pragma_repeated_operation_mapping() {
    let operation = Operation::from(PragmaRepeatedMeasurement::new(
        "ro".to_string(),
        1,
        Some(tmp_create_map()),
    ));
    let qasm_string = call_operation(&operation, "q", QasmVersion::V2point0).unwrap();
    assert!(qasm_string.contains("measure q[0] -> ro[1];\n"));
    assert!(qasm_string.contains("measure q[1] -> ro[0];\n"));
    let qasm_string =
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap();
    assert!(qasm_string.contains("measure q[0] -> ro[1];\n"));
    assert!(qasm_string.contains("measure q[1] -> ro[0];\n"));
    let qasm_string = call_operation(
        &operation,
        "q",
        QasmVersion::V3point0(Qasm3Dialect::Vanilla),
    )
    .unwrap();
    assert!(qasm_string.contains("measure q[0] -> ro[1];\n"));
    assert!(qasm_string.contains("measure q[1] -> ro[0];\n"));
    let qasm_string =
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap();
    assert!(qasm_string.contains("measure q[0] -> ro[1];\n"));
    assert!(qasm_string.contains("measure q[1] -> ro[0];\n"));
}

/// Test that non-included gates return an error
#[test_case(Operation::from(Bogoliubov::new(
    0,
    1,
    CalculatorFloat::from(0.2),
    CalculatorFloat::from(0.2),
)); "Bogoliubov")]
fn test_call_operation_error(operation: Operation) {
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V2point0),
        Err(RoqoqoBackendError::OperationNotInBackend {
            backend: "QASM",
            hqslang: operation.hqslang()
        })
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Braket)),
        Err(RoqoqoBackendError::OperationNotInBackend {
            backend: "QASM",
            hqslang: operation.hqslang()
        })
    );
    assert_eq!(
        call_operation(
            &operation,
            "q",
            QasmVersion::V3point0(Qasm3Dialect::Vanilla)
        ),
        Err(RoqoqoBackendError::OperationNotInBackend {
            backend: "QASM",
            hqslang: operation.hqslang()
        })
    );
    assert_eq!(
        call_operation(&operation, "q", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)),
        Err(RoqoqoBackendError::OperationNotInBackend {
            backend: "QASM",
            hqslang: operation.hqslang()
        })
    );
}

/// Test that a circuit can be correctly translated
#[test]
fn test_call_circuit() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 1, false);
    circuit += PauliX::new(0);
    circuit += MeasureQubit::new(0, "ro".to_string(), 0);

    let qasm_circ_2_0: Vec<String> = vec![
        "creg ro[1];".to_string(),
        "x qr[0];".to_string(),
        "measure qr[0] -> ro[0];".to_string(),
    ];
    assert_eq!(
        call_circuit(&circuit, "qr", QasmVersion::V2point0).unwrap(),
        qasm_circ_2_0
    );

    let qasm_circ_3_0: Vec<String> = vec![
        "bit[1] ro;".to_string(),
        "x qr[0];".to_string(),
        "measure qr[0] -> ro[0];".to_string(),
    ];
    assert_eq!(
        call_circuit(&circuit, "qr", QasmVersion::V3point0(Qasm3Dialect::Braket)).unwrap(),
        qasm_circ_3_0
    );
    assert_eq!(
        call_circuit(&circuit, "qr", QasmVersion::V3point0(Qasm3Dialect::Vanilla)).unwrap(),
        qasm_circ_3_0
    );
    assert_eq!(
        call_circuit(&circuit, "qr", QasmVersion::V3point0(Qasm3Dialect::Roqoqo)).unwrap(),
        qasm_circ_3_0
    );
}


#[test]
fn test_parametric_gates() {
    let op = Operation::from(RotateZ::new(0, "2.44+ciaoz/4*theta".into()));

    let output = call_operation(&op, "q", QasmVersion::V3point0(Qasm3Dialect::Vanilla)).unwrap();

    println!("{output}");
}