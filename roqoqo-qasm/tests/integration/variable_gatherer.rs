use qoqo_calculator::CalculatorFloat;
use roqoqo::{operations::*, Circuit};

use roqoqo_qasm::Backend;
use roqoqo_qasm::VariableGatherer;

#[test]
fn testin_str() {
    let calc_0 = CalculatorFloat::from("2*(a+1)");
    let calc_1 = CalculatorFloat::from("cos(b)+4");
    let mut cp = VariableGatherer::new();
    let res = cp.parse(&calc_0.to_string()).unwrap();
    println!("{:?}", res);
    let res = cp.parse(&calc_1.to_string()).unwrap();
    println!("{:?}", res);
    println!("{:?}", cp.variables);
}

#[test]
fn test_backend() {
    let backend = Backend::new(Some("qr".to_string()), Some("3.0".to_string())).unwrap();
    let mut circuit = Circuit::new();
    circuit += RotateZ::new(0, "2*(a+1)".into());
    println!("{}", backend.circuit_to_qasm_str(&circuit).unwrap());
}
