// Copyright 2019 Q1t BV
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate ndarray;
extern crate num_complex;

use cmatrix;
use gates;
use qasm;

/// The Hadamard gate.
///
/// The Hadamard gate maps the zero state |0&rang; to the symmetric combination
/// of |0&rang; and |1&rang;, and the |1&rang; state to the anti-symmetric
/// combination.
pub struct H
{
}

impl H
{
    /// Create a new Hadamard gate.
    pub fn new() -> Self
    {
        H { }
    }

    pub fn transform(state: &mut cmatrix::CVecSliceMut)
    {
        assert!(state.len() % 2 == 0, "Number of rows is not even.");

        let n = state.len() / 2;
        let s1_copy = state.slice(s![n..]).to_owned();

        let (mut s0, mut s1) = state.view_mut().split_at(ndarray::Axis(0), n);

        s1 -= &s0;
        s1 *= -cmatrix::COMPLEX_HSQRT2;
        s0 += &s1_copy;
        s0 *= cmatrix::COMPLEX_HSQRT2;
    }

    pub fn cost() -> f64
    {
        gates::U2::cost()
    }

}

impl gates::Gate for H
{
    fn cost(&self) -> f64
    {
        Self::cost()
    }

    fn description(&self) -> &str
    {
        "H"
    }

    fn nr_affected_bits(&self) -> usize
    {
        1
    }

    fn matrix(&self) -> cmatrix::CMatrix
    {
        let x = cmatrix::COMPLEX_HSQRT2;
        array![[x, x], [x, -x]]
    }

    fn apply_slice(&self, state: &mut cmatrix::CVecSliceMut)
    {
        Self::transform(state);
    }
}

impl qasm::OpenQasm for H
{
    fn open_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("h {}", bit_names[bits[0]])
    }
}

impl qasm::CQasm for H
{
    fn c_qasm(&self, bit_names: &[String], bits: &[usize]) -> String
    {
        format!("h {}", bit_names[bits[0]])
    }
}

#[cfg(test)]
mod tests
{
    use gates::{gate_test, Gate, H};
    use qasm::{OpenQasm, CQasm};
    use cmatrix;

    #[test]
    fn test_description()
    {
        let h = H::new();
        assert_eq!(h.description(), "H");
    }

    #[test]
    fn test_matrix()
    {
        let h = H::new();
        let s = cmatrix::COMPLEX_HSQRT2;
        assert_complex_matrix_eq!(h.matrix(), array![[s, s], [s, -s]]);
    }

    #[test]
    fn test_apply()
    {
        let z = cmatrix::COMPLEX_ZERO;
        let o = cmatrix::COMPLEX_ONE;
        let x = cmatrix::COMPLEX_HSQRT2;
        let mut state = array![[o, z, x, x], [z, o, x, -x]];
        let result = array![[x, x, o, z], [x, -x, z, o]];
        gate_test(H::new(), &mut state, &result);
    }

    #[test]
    fn test_open_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = H::new().open_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "h qb");
    }

    #[test]
    fn test_c_qasm()
    {
        let bit_names = [String::from("qb")];
        let qasm = H::new().c_qasm(&bit_names, &[0]);
        assert_eq!(qasm, "h qb");
    }
}
