# Changelog

This changelog track changes to the qoqo qasm project starting at version 0.5.0

### 0.12.5

* Added support for `EchoCrossResonance`

### 0.12.4

* Fixed atomicity rule for expressions
* Added `r` symbol to the parser for the `RotateXY` operation

### 0.12.3

* Handle errors better for Qulacs dialect.

### 0.12.2

* Added support for Qulacs specific qasm.

### 0.12.1

* Added support for `SqrtPauliY` and `InvSqrtPauliY`
* Updated to qoqo 1.15

### 0.12.0

* Added support for `ControlledRotateX` and `ControlledRotateXY`

### 0.11.2

* Fixed build workflows issues

### 0.11.1

* Fixed outdated dependencies

### 0.11.0

* Added support for GateDefinition and CallDefinedGate
* Updated to pyo3 0.21
* Updated to qoqo 1.14

### 0.10.1

* Fixed missing '\n' character bug for QASM parsing feature

### 0.10.0

* Updated QASM parsing feature to handle mathematical expressions and symbols correctly
* QASM files/strings parsing feature is now stable

### 0.9.7

* Removed `output` keyword from qasm string for 3.0Braket when the Definition has
`is_output=True` as AWS Braket does not support this functionality.

### 0.9.6

* Modified QASM 2.0 `PragmaSleep` gate definition

### 0.9.5

* Updated QASM parsing feature to skip include files lines

### 0.9.4

* Updated to qoqo 1.9
* Updated QASM parsing feature to skip gate definitions

### 0.9.3

* Updated to qoqo 1.8
* Updated to Pyo3 0.20

### 0.9.2

* Added GPi and GPi2 QASM definitions

### 0.9.1

* Updated to qoqo 1.7

### 0.9.0

* Added support for parametric gates using OpenQASM 3.0

### 0.8.3

* Bugfix for PragmaLoop in 3.0Braket and 2.0

### 0.8.2

* Bugfix for PragmaGlobalPhase in 3.0Braket

### 0.8.1

* Updated to qoqo 1.5.1
* Updated to pyo3 0.19

### 0.8.0

* Added QASM files/strings parsing feature

### 0.7.5

* Added support for 3-qubit operations

### 0.7.4

* Modified braket version of 3.0 to work with amazon braket

### 0.7.3

* Updated to qoqo 1.4

### 0.7.2

* Modified cx definition for 3.0

### 0.7.1

* Updated to qoqo 1.3

### 0.7.0

* Extended support for OpenQASM 3.0 features

### 0.6.0

* Added support for OpenQASM 3.0

### 0.5.1

* Added support for most qoqo Operations
* Removed qelib1.inc import, added gate_definition method
* Updated dev dependency

### 0.5.0

* Updated to qoqo 1.2, refactored qoqo_qasm codebase as roqoqo_qasm Python interface

### 0.4.6

* Updated to qoqo 1.1.0, qoqo_calculator_pyo3 1.1.0

### 0.4.5

* Updated to qoqo 1.0.0, qoqo_calculator_pyo3 1.0.0

### 0.4.4

* Updated to qoqo 1.0.0-alpha.5

### 0.4.3

* Updated to qoqo 1.0.0-alpha.2 and qoqo_calculator 0.8.3

## 0.4.2

* qoqo_qasm can now be built using a source distribution
* Removed support for pyhton 3.6
* Updated dependencies to latest

## 0.4.0

* Updated qoqo/roqoqo dependencies to 0.6
