# How to contribute

We are happy to include your contribution to this project. To contribute open a pull request and we will get back to you.

## Contributor License Agreement

To clarify the intellectual property license granted with Contributions from any person or entity to HQS, we must have a Contributor License Agreement ("CLA") in place with each contributor. This license is for your protection as a Contributor as well as the protection of HQS and the users of this project; it does not change your rights to use your own Contributions for any other purpose.

Please fill and sign the CLA found at *url* and send it to info@quantumsimulations.de.

## Code Guidelines for Python

1. Testing: We use pytest for this project. We require that all previous tests pass and that your provide proper tests with your contribution.
2. MyPy: We use type annotations and mypy to check for proper type annotations and usage of types throughout the code.
3. Linting: We use flake8 with the configuration in .flake8.

## Code Guidelines for Rust

1. Testing: We use `cargo test` (in qoqo_qasm/roqoqo_qasm) for roqoqo_qasm and `pytest ./tests` (in qoqo_qasm/qoqo_qasm) for qoqo_qasm. We require that all previous tests pass and that your provide proper tests with your contribution.
2. Linting: We use `cargo clippy -- -D warnings` to lint all Rust code (qoqo_qasm/roqoqo_qasm), and `flake8 ./qoqo_qasm`, `mypy ./qoqo_qasm` to lint all Python code (qoqo_qasm/qoqo_qasm).
3. Formatting: We check formatting with `cargo fmt --all --check` in Rust code (qoqo_qasm/roqoqo_qasm), and with flake8 in Python code (qoqo_qasm/qoqo_qasm).
