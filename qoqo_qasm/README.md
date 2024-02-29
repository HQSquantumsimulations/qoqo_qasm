# qoqo-qasm

[![Documentation Status](https://img.shields.io/badge/docs-read-blue)](https://hqsquantumsimulations.github.io/qoqo_qasm/)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qasm/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qasm/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo_qasm)](https://pypi.org/project/qoqo_qasm/)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_qasm)](https://pypi.org/project/qoqo_qasm/)
![Crates.io](https://img.shields.io/crates/l/qoqo-qasm)

Qasm interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-qasm provides the QasmBackend class that allows users translate a qoqo circuit into a qasm file.
Not all qoqo operations have a corresponding qasm expression.  
Circuits containing operations without a corresponding expression can not be translated.

A source distribution now exists but requires a Rust install with a rust version > 1.47 and a maturin version { >= 0.14, <0.15 } in order to be built.

## General Notes

This project is partly supported by [PlanQK](https://planqk.de).

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
