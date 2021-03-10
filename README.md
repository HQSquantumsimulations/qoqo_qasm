# qoqo-qasm interface

![Read the Docs](https://img.shields.io/readthedocs/qoqo_qasm)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/HQSquantumsimulations/qoqo_qasm/ci_tests)
![PyPI](https://img.shields.io/pypi/v/qoqo_qasm)
![GitHub](https://img.shields.io/github/license/HQSquantumsimulations/qoqo_qasm)
![PyPI - Format](https://img.shields.io/pypi/format/qoqo_qasm)

Qasm interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-qasm provides the QasmBackend class that allows users translate a qoqo circuit into a qasm file.
Not all qoqo operations have a corresponding qasm expression.  
Circuits containing operations without a corresponding expression can not be translated.

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.
