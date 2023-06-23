OPENQASM 2.0;
creg c[2];
qreg q[2];

rz(0.2) q[0];
// ry(0.3) q[1];
rx(2.1) q[2];
h q[0];
// x q[2];
y q[1];
