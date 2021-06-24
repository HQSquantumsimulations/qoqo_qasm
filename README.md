<img src="qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# qoqo-qasm interface

[![Documentation Status](https://readthedocs.org/projects/qoqo_qasm/badge/?version=latest)](https://qoqo_qasm.readthedocs.io/en/latest/?badge=latest)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qasm/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qasm/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo_qasm)](https://pypi.org/project/qoqo_qasm/)
![PyPI - License](https://img.shields.io/pypi/l/qoqo_qasm)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_qasm)](https://pypi.org/project/qoqo_qasm/)

Qasm interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-qasm provides the QasmBackend class that allows users translate a qoqo circuit into a qasm file.
Not all qoqo operations have a corresponding qasm expression.  
Circuits containing operations without a corresponding expression can not be translated.

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.
