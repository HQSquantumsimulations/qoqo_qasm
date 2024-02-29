OPENQASM 2.0;
creg c[3];
qreg q[3];

rz(pi) q[0];
rx(pi/2) q[1];
ry(pi/4) q[2];
p((2*3.5)/2) q[2];
p(cos(pi)) q[1];
cp(1+sin(-pi/2)+2.5) q[0],q[1];
rz(tan(pi/3)) q[2];
rz(exp(0.5+0.5)) q[1];
rz(ln(exp(1))) q[0];
rx(2*sqrt(4)) q[1];
