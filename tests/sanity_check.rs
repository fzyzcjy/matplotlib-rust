use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[test]
fn python_sanity_check() -> Result<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}

#[test]
fn matplotlib_sanity_check() -> Result<()> {
    Python::with_gil(|py| {
        println!("call import");
        let plt = py.import("matplotlib.pyplot")?;
        println!("result {:?}", plt);

        println!("call plot");
        println!("result {:?}", plt.call_method1("plot", (10, 20))?);

        println!("call show");
        println!("result {:?}", plt.call_method0("show")?);

        Ok(())
    })
}
