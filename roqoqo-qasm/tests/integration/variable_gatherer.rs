use qoqo_calculator::CalculatorFloat;

use roqoqo_qasm::VariableGatherer;

use test_case::test_case;

/// Test single CalculatorFloat
#[test]
fn test_single_cf() {
    let calc_0 = CalculatorFloat::from("2*(a+1)");

    let mut cp = VariableGatherer::new();
    cp.parse(&calc_0.to_string()).unwrap();

    assert_eq!(cp.variables.len(), 1);
    assert!(cp.variables.contains("a"));
}

/// Test CalculatorFloat sequence
#[test]
fn test_multiple_cf() {
    let calc_0 = CalculatorFloat::from("2*(a+1)");
    let calc_1 = CalculatorFloat::from("b");
    let calc_2 = CalculatorFloat::from(2.0);
    let calc_3 = CalculatorFloat::from("2*(c+1)");

    let mut cp = VariableGatherer::new();
    cp.parse(&calc_0.to_string()).unwrap();
    cp.parse(&calc_1.to_string()).unwrap();
    cp.parse(&calc_2.to_string()).unwrap();
    cp.parse(&calc_3.to_string()).unwrap();

    assert_eq!(cp.variables.len(), 3);
    assert!(cp.variables.contains("a"));
    assert!(cp.variables.contains("b"));
    assert!(cp.variables.contains("c"));
}

/// Test non-supported mathematical functions
#[test_case(CalculatorFloat::from("2*abs(a+1)"), "abs")]
#[test_case(CalculatorFloat::from("2*cosh(a+1)"), "cosh")]
#[test_case(CalculatorFloat::from("2*sinh(a+1)"), "sinh")]
#[test_case(CalculatorFloat::from("2*tanh(a+1)"), "tanh")]
#[test_case(CalculatorFloat::from("2*acosh(a+1)"), "acosh")]
#[test_case(CalculatorFloat::from("2*asinh(a+1)"), "asinh")]
#[test_case(CalculatorFloat::from("2*atanh(a+1)"), "atanh")]
#[test_case(CalculatorFloat::from("2*arcosh(a+1)"), "arcosh")]
#[test_case(CalculatorFloat::from("2*arsinh(a+1)"), "arsinh")]
#[test_case(CalculatorFloat::from("2*artanh(a+1)"), "artanh")]
#[test_case(CalculatorFloat::from("2*exp2(a+1)"), "exp2")]
#[test_case(CalculatorFloat::from("2*expm1(a+1)"), "expm1")]
#[test_case(CalculatorFloat::from("2*log10(a+1)"), "log10")]
#[test_case(CalculatorFloat::from("2*cbrt(a+1)"), "cbrt")]
#[test_case(CalculatorFloat::from("2*fract(a+1)"), "fract")]
#[test_case(CalculatorFloat::from("2*round(a+1)"), "round")]
#[test_case(CalculatorFloat::from("2*erf(a+1)"), "erf")]
#[test_case(CalculatorFloat::from("2*tgamma(a+1)"), "tgamma")]
#[test_case(CalculatorFloat::from("2*lgamma(a+1)"), "lgamma")]
#[test_case(CalculatorFloat::from("2*delta(a+1)"), "delta")]
#[test_case(CalculatorFloat::from("2*theta(a+1)"), "theta")]
#[test_case(CalculatorFloat::from("2*parity(a+1)"), "parity")]
#[test_case(CalculatorFloat::from("2*atan2(a+1)"), "atan2")]
#[test_case(CalculatorFloat::from("2*hypot(a+1)"), "hypot")]
#[test_case(CalculatorFloat::from("2*max(a+1)"), "max")]
#[test_case(CalculatorFloat::from("2*min(a+1)"), "min")]
fn test_math_functions_errors(cf: CalculatorFloat, name: &str) {
    let mut cp = VariableGatherer::new();

    let incorrect_parse = cp.parse(&cf.to_string());

    assert!(incorrect_parse.is_err());
    assert!(incorrect_parse.unwrap_err().to_string().contains(&format!(
        "Function {name} is not supported in OpenQASM 3.0."
    )));
}

/// Test supported mathematical functions
#[test_case(CalculatorFloat::from("2*sin(a+1)"))]
#[test_case(CalculatorFloat::from("2*cos(a+1)"))]
#[test_case(CalculatorFloat::from("2*tan(a+1)"))]
#[test_case(CalculatorFloat::from("2*acos(a+1)"))]
#[test_case(CalculatorFloat::from("2*asin(a+1)"))]
#[test_case(CalculatorFloat::from("2*atan(a+1)"))]
#[test_case(CalculatorFloat::from("2*exp(a+1)"))]
#[test_case(CalculatorFloat::from("2*log(a+1)"))]
#[test_case(CalculatorFloat::from("2*sqrt(a+1)"))]
#[test_case(CalculatorFloat::from("2*ceil(a+1)"))]
#[test_case(CalculatorFloat::from("2*floor(a+1)"))]
#[test_case(CalculatorFloat::from("2*sign(a+1)"))]
#[test_case(CalculatorFloat::from("2*pow(2, a+1)"))]
fn test_math_functions(cf: CalculatorFloat) {
    let mut cp = VariableGatherer::new();

    let correct_parse = cp.parse(&cf.to_string());

    assert!(correct_parse.is_ok());
}
