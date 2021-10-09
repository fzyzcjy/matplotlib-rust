use anyhow::Result;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use widestring::WideCString;

use libc::wchar_t;

#[test]
fn python_sanity_check() -> Result<()> {
    if let Some(python_home) = std::env::var_os("CONDA_PREFIX") {
        unsafe {
            ffi::Py_SetPythonHome(
                WideCString::from_str(python_home.to_str().unwrap())
                    .unwrap()
                    .as_ptr() as *const wchar_t,
            );
        }
    }

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
