#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene, decl_macro)]
#![feature(try_trait)]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;

pub mod common;
pub mod inven;
// 可以做成whl格式，太方便了pip install maturin  根目录maturin build
// maturin build --interpreter python

#[pymodule]
fn ruPyInven(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "inven_everyday")]
    fn inven_everyday(py: Python, raw: &str, days: &str) -> PyResult<String> {
        let res = inven::everyday::inven_everyday(raw, days);
        Ok(res)
    }
    #[pyfn(m, "value_everyday")]
    fn value_everyday(py: Python, raw: &str, days: &str, price: &str) -> PyResult<String> {
        let res = inven::everyday::value_everyday(raw, days, price);
        Ok(res)
    }
    #[pyfn(m, "price_everyday")]
    fn price_everyday(py: Python,  days: &str, price: &str) -> PyResult<String> {
        let res = inven::everyday::price_everyday(days, price);
        Ok(res)
    }
    Ok(())
}
// #[pyfunction]
// fn inven_everyday(raw: &str, days: &str) -> PyResult<String> {
//     let res = inven::everyday::inven_everyday(raw, days);
//     Ok(res)
// }
//
// #[pyfunction]
// fn value_everyday(raw: &str, days: &str, price: &str) -> PyResult<String> {
//     let res = inven::everyday::value_everyday(raw, days, price);
//     Ok(res)
// }
//
// #[pymodule]
// fn ruPyInven(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(int_str))?;
//     m.add_wrapped(wrap_pyfunction!(str_str))?;
//     Ok(())
// }
