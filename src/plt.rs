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

type Args = impl IntoPy<Py<PyTuple>>;
type KwArgs<'a> = Option<&'a PyDict>;

impl PyPlot<'_> {
    pub fn subplot(&self, nrows: i32, ncols: i32, index: i32, kwargs: KwArgs) -> Result<()> {
        eat_response(self.0.call_method("subplot", (nrows, ncols, index), kwargs))
    }

    pub fn plot(&self, args: Args, kwargs: KwArgs) -> Result<()> {
        eat_response(self.0.call_method("plot", args, kwargs))
    }

    pub fn imshow(&self, args: Args, kwargs: KwArgs) -> Result<()> {
        eat_response(self.0.call_method("imshow", args, kwargs))
    }

    pub fn tight_layout(&self, args: Args, kwargs: KwArgs) -> Result<()> {
        eat_response(self.0.call_method("tight_layout", args, kwargs))
    }

    pub fn show(&self) -> Result<()> {
        eat_response(self.0.call_method0("show"))
    }
}

fn eat_response<R>(raw: PyResult<R>) -> Result<()> {
    raw?;
    Ok(())
}
