# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Install qoqo_qiskit"""

from setuptools import find_packages, setup
import os
path = os.path.dirname(os.path.abspath(__file__))

__version__ = None
with open(os.path.join(path, 'qoqo-qiskit/__version__.py')) as f:
    lines = f.readlines()
__version__ = lines[-1].strip().split("'")[1].strip()

install_requires = [
    'numpy',
    'qoqo>=1.2',
    'qoqo_qasm>=0.5',
    'qiskit'
]

setup(name='qoqo_qiskit',
    version=__version__,
    python_requires='>=3.7',
    packages=find_packages(exclude=('docs')),
    install_requires=install_requires
    )
