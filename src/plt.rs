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

macro_rules! fn_call_method0 {
    ($func_name:ident) => {
        pub fn $func_name(&self) -> Result<()> {
            eat_response(self.plt.call_method0(stringify!($func_name)))
        }
    };
}

macro_rules! fn_call_method1 {
    ($func_name:ident) => {
        pub fn $func_name(&self, args: impl IntoPy<Py<PyTuple>>) -> Result<()> {
            eat_response(self.plt.call_method1(stringify!($func_name), args))
        }
    };
}

macro_rules! fn_call_method {
    ($func_name:ident) => {
        pub fn $func_name(&self, args: impl IntoPy<Py<PyTuple>>, kwargs: KwArgs) -> Result<()> {
            eat_response(self.plt.call_method(stringify!($func_name), args, kwargs))
        }
    };
}

impl PyPlot<'_> {
    pub fn subplot(&self, nrows: i32, ncols: i32, index: i32, kwargs: KwArgs) -> Result<()> {
        eat_response(
            self.plt
                .call_method("subplot", (nrows, ncols, index), kwargs),
        )
    }

    fn_call_method!(plot);
    fn_call_method!(imshow);

    fn_call_method0!(tight_layout);
    fn_call_method0!(colorbar);
    fn_call_method0!(show);
}

fn eat_response<R>(raw: PyResult<R>) -> Result<()> {
    raw?;
    Ok(())
}
