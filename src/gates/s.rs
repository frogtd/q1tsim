extern crate num_complex;

use cmatrix;
use gates;
use qasm;

/// The Clifford `S` gate
///
/// The `S` gate rotates the state over π/2 radians around the `z` axis of
/// the Bloch sphere. It is the square root of the `Z` gate.
pub struct S
{
}

impl S
{
    /// Create a new `S` gate.
    pub fn new() -> Self
    {
        S { }
    }
}

impl gates::Gate for S
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
    }

    fn description(&self) -> &str
    {
        "S"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let i = cmatrix::COMPLEX_I;
        array![[o, z], [z, i]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let mut slice = state.slice_mut(s![n..]);
        slice *= cmatrix::COMPLEX_I;
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        let mut slice = state.slice_mut(s![n.., ..]);
        slice *= cmatrix::COMPLEX_I;
    }
}

impl qasm::OpenQasm for S
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("s {}", bit_names[bits[0]])
    }
}

impl qasm::CQasm for S
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("s {}", bit_names[bits[0]])
    }
}

/// Conjugate of Clifford `S` gate
///
/// The `S`<sup>`†`</sup> gate rotates the state over -π/2 radians around the
/// `z` axis of the Bloch sphere. It is the conjugate of the `S` gate.
pub struct Sdg
{
}

impl Sdg
{
    /// Create a new `S`<sup>`†`</sup> gate.
    pub fn new() -> Self
    {
        Sdg { }
    }
}

impl gates::Gate for Sdg
{
    fn cost(&self) -> f64
    {
        gates::U1::cost()
    }

    fn description(&self) -> &str
    {
        "S†"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let i = cmatrix::COMPLEX_I;
        array![[o, z], [z, -i]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let mut slice = state.slice_mut(s![n..]);
        slice *= -cmatrix::COMPLEX_I;
    }

    fn apply_mat_slice(&self, state: &mut cmatrix::CMatSliceMut)
    {
        assert!(state.rows() % 2 == 0, "Number of rows is not even.");

        let n = state.rows() / 2;
        let mut slice = state.slice_mut(s![n.., ..]);
        slice *= -cmatrix::COMPLEX_I;
    }
}

impl qasm::OpenQasm for Sdg
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("sdg {}", bit_names[bits[0]])
    }
}

impl qasm::CQasm for Sdg
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("sdag {}", bit_names[bits[0]])
    }
}

#[cfg(test)]
mod tests
{
    use gates::{gate_test, Gate, S, Sdg};
    use qasm::{OpenQasm, CQasm};
    use cmatrix;

    #[test]
    fn test_description()
    {
        let gate = S::new();
        assert_eq!(gate.description(), "S");
        let gate = Sdg::new();
        assert_eq!(gate.description(), "S†");
    }

    #[test]
    fn test_matrix()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let i = cmatrix::COMPLEX_I;

        let gate = S::new();
        assert_complex_matrix_eq!(gate.matrix(), array![[o, z], [z, i]]);

        let gate = Sdg::new();
        assert_complex_matrix_eq!(gate.matrix(), array![[o, z], [z, -i]]);
    }

    #[test]
    fn test_apply()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let i = cmatrix::COMPLEX_I;
        let x = cmatrix::COMPLEX_HSQRT2;

        let mut state = array![
            [o, z, x,  x],
            [z, o, x, -x]
        ];
        let result = array![
            [o, z,   x,    x],
            [z, i, x*i, -x*i]
        ];
        gate_test(S::new(), &mut state, &result);

        let mut state = array![
            [o, z, x,  x],
            [z, o, x, -x]
        ];
        let result = array![
            [o,  z,    x,   x],
            [z, -i, -x*i, x*i]
        ];
        gate_test(Sdg::new(), &mut state, &result);
    }

    #[test]
    fn test_open_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = S::new().open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "s qb");
        let qasm = Sdg::new().open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "sdg qb");
    }

    #[test]
    fn test_c_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = S::new().c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "s qb");
        let qasm = Sdg::new().c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "sdag qb");
    }
}
