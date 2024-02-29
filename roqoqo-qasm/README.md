# roqoqo-qasm

[![Crates.io](https://img.shields.io/crates/v/roqoqo-qasm)](https://crates.io/crates/roqoqo-qasm)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qasm/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qasm/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-qasm)](https://docs.rs/roqoqo-qasm/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qasm)

Qasm interface for the roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

roqoqo-qasm provides the QasmBackend class that allows users translate a roqoqo circuit into a qasm file.
Not all roqoqo operations have a corresponding qasm expression.  
Circuits containing operations without a corresponding expression can not be translated.

## General Notes

This project is partly supported by [PlanQK](https://planqk.de).

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
