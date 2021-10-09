use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::*;

pub struct PyPlot<'p> {
    py: Python<'p>,
    plt: &'p PyModule,
}

impl<'p> PyPlot<'p> {
    pub fn with_gil<F, R>(f: F) -> Result<R>
    where
        F: FnOnce(PyPlot) -> Result<R>,
    {
        Python::with_gil(|py| -> Result<R> { f(PyPlot::try_new(py)?) })
    }

    pub fn try_new(py: Python<'p>) -> Result<Self> {
        Ok(Self {
            py,
            plt: py.import("matplotlib.pyplot")?,
        })
    }

    pub fn py(&self) -> Python<'p> {
        self.py
    }
}

type KwArgs<'a> = Option<&'a PyDict>;

impl PyPlot<'_> {
    pub fn subplot(&self, nrows: i32, ncols: i32, index: i32, kwargs: KwArgs) -> Result<()> {
        eat_response(
            self.plt
                .call_method("subplot", (nrows, ncols, index), kwargs),
        )
    }

    pub fn plot(&self, args: impl IntoPy<Py<PyTuple>>, kwargs: KwArgs) -> Result<()> {
        eat_response(self.plt.call_method("plot", args, kwargs))
    }

    pub fn imshow(&self, args: impl IntoPy<Py<PyTuple>>, kwargs: KwArgs) -> Result<()> {
        eat_response(self.plt.call_method("imshow", args, kwargs))
    }

    pub fn tight_layout(&self, args: impl IntoPy<Py<PyTuple>>, kwargs: KwArgs) -> Result<()> {
        eat_response(self.plt.call_method("tight_layout", args, kwargs))
    }

    pub fn show(&self) -> Result<()> {
        eat_response(self.plt.call_method0("show"))
    }
}

fn eat_response<R>(raw: PyResult<R>) -> Result<()> {
    raw?;
    Ok(())
}
