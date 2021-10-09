use anyhow::Result;
use libc::wchar_t;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use widestring::WideCString;

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
