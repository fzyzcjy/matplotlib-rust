use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::*;

pub struct PyPlot<'p>(&'p PyModule);

impl<'p> PyPlot<'p> {
    pub fn with_gil<F, R>(f: F) -> Result<R>
    where
        F: FnOnce(PyPlot) -> Result<R>,
    {
        Python::with_gil(|py| -> Result<R> { f(PyPlot::try_new(py)?) })
    }

    pub fn try_new(py: Python<'p>) -> Result<Self> {
        Ok(Self(py.import("matplotlib.pyplot")?))
    }
}

impl PyPlot<'_> {
    pub fn plot(&self, args: impl IntoPy<Py<PyTuple>>, kwargs: Option<&PyDict>) -> Result<()> {
        eat_response(self.0.call_method("plot", args, kwargs))
    }

    pub fn show(&self) -> Result<()> {
        eat_response(self.0.call_method0("show"))
    }
}

fn eat_response<R>(raw: PyResult<R>) -> Result<()> {
    raw?;
    Ok(())
}
