OPENQASM 2.0;
creg c[2];
qreg q[3];

x q[0];
h q[1];
rx(2.3) q[2];
cx q[0],q[1];

measure q[0] -> c[0];
