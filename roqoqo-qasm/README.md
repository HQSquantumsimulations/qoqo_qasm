<img src="../qoqo_Logo_vertical_color.png" alt="qoqo logo" width="300" />

# qoqo-qasm

Qasm interface for the qoqo/roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

This repository contains two components:

* The qoqo_qasm backend for the qoqo python interface to roqoqo
* The roqoqo_qasm backend for roqoqo directly

## qoqo-qasm

[![Documentation Status](https://readthedocs.org/projects/qoqo-qasm/badge/?version=latest)](https://qoqo-qasm.readthedocs.io/en/latest/?badge=latest)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qasm/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qasm/actions)
[![PyPI](https://img.shields.io/pypi/v/qoqo_qasm)](https://pypi.org/project/qoqo_qasm/)
![PyPI - License](https://img.shields.io/pypi/l/qoqo_qasm)
[![PyPI - Format](https://img.shields.io/pypi/format/qoqo_qasm)](https://pypi.org/project/qoqo_qasm/)

Qasm interface for the qoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

qoqo-qasm provides the QasmBackend class that allows users translate a qoqo circuit into a qasm file.
Not all qoqo operations have a corresponding qasm expression.  
Circuits containing operations without a corresponding expression can not be translated.

A source distribution now exists but requires a Rust install with a rust version > 1.47 and a maturin version { >= 0.14, <0.15 } in order to be built.

## roqoqo-qasm

[![Crates.io](https://img.shields.io/crates/v/roqoqo-qasm)](https://crates.io/crates/roqoqo-qasm)
[![GitHub Workflow Status](https://github.com/HQSquantumsimulations/qoqo_qasm/workflows/ci_tests/badge.svg)](https://github.com/HQSquantumsimulations/qoqo_qasm/actions)
[![docs.rs](https://img.shields.io/docsrs/roqoqo-qasm)](https://docs.rs/roqoqo-qasm/)
![Crates.io](https://img.shields.io/crates/l/roqoqo-qasm)

Qasm interface for the roqoqo quantum toolkit by [HQS Quantum Simulations](https://quantumsimulations.de).

roqoqo-qasm provides the QasmBackend class that allows users translate a roqoqo circuit into a qasm file.
Not all roqoqo operations have a corresponding qasm expression.  
Circuits containing operations without a corresponding expression can not be translated.

## General Notes

This software is still in the beta stage. Functions and documentation are not yet complete and breaking changes can occur.

This project is partly supported by [PlanQK](https://planqk.de).

## Contributing

We welcome contributions to the project. If you want to contribute code, please have a look at CONTRIBUTE.md for our code contribution guidelines.
