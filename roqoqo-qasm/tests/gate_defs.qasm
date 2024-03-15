OPENQASM 2.0;

gate u3(theta,phi,lambda) q { U(theta,phi,lambda) q; }
gate u2(phi,lambda) q { U(pi/2,phi,lambda) q; }
gate u1(lambda) q { U(0,0,lambda) q; }
gate rx(theta) a { u3(theta,-pi/2,pi/2) a; }
gate ry(theta) a { u3(theta,0,0) a; }
gate rz(phi) a { u1(phi) a; }
gate cx c,t { CX c,t; }
gate h a { u2(0,pi) a; }

gate custom_gate(theta, phi) qb1,qb2
{
    h qb1;
    rx(theta,phi) qb2;
    rx(phi*pi/2) qb1;
}

qreg q[1];

h q[0];
custom_gate(0.5,pi) q[0],q[1];
