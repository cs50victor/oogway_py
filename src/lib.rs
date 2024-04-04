use std::time::Duration;

use ::oogway::Oogway as Oogwayy;
use pyo3::{exceptions::PyKeyError, prelude::*};

/// A Python module implemented in Rust.
#[pymodule]
fn oogway(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Oogway>()?;
    Ok(())
}


// not really optimized , prolly a better way to do this
#[pyclass]
pub struct Oogway {
    inner: Oogwayy
}

#[pymethods]
impl Oogway {
    #[new]
    fn new() -> PyResult<Oogway> {
        match Oogwayy::new() {
            Ok(oogway) => Ok(Self{inner : oogway }),
            Err(e) => Err(PyKeyError::new_err(e.to_string())),
        }
    }

    #[setter(model_name)]
    fn set_model_name(&mut self, model_name: String){
        self.inner.model(model_name);
    }

    pub fn ask<'a >(&mut self, py: Python<'a>, question: String) -> PyResult<&'a PyAny> {
        let x = self.inner.ask(question);
        // x
        pyo3_asyncio::async_std::into_coroutine(py, async move {
            async_std::task::sleep(Duration::from_secs(1)).await;
            Ok(())
        })
        // pyo3_asyncio::tokio::future_into_py(py, async move {
        //     async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        //     Ok(Python::with_gil(|py| py.None()))
        // })

        // pyo3_asyncio::async_std::into_coroutine(py, async move {
        //     let x = self.inner.ask(question);
        //     let x = x.await;
        //     Ok(())
        // })
    }

}
