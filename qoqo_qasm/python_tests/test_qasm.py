# Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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

import pytest
import sys
from qoqo_qasm import QasmBackend
from qoqo import Circuit
from qoqo import operations as ops


def test_qasm() -> None:
    circuit = Circuit()
    circuit += ops.Hadamard(0)

    backend = QasmBackend(None, "3.0")
    qasm = backend.circuit_to_qasm_str(circuit)
    assert qasm


if __name__ == "__main__":
    pytest.main(sys.argv)
