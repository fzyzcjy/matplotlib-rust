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
    Python::with_gil(|py| -> Result<()> {
        // println!("call import matplotlib");
        // let matplotlib = py.import("matplotlib")?;
        // println!("result {:?}", matplotlib);
        //
        // println!("call get_backend");
        // println!("result {:?}", matplotlib.call_method0("get_backend")?);
        //
        // println!("call version");
        // println!("result {:?}", matplotlib.getattr("__version__")?);

        println!(
            "result {:?}",
            py.run(
                "\
import matplotlib.pyplot as plt;
print('after import');
plt.plot([10,20], [100,110]);
print('after plot');
plt.show();
print('after show');
            ",
                None,
                None,
            )?
        );

        // println!(
        //     "result {:?}",
        //     py.run("import matplotlib.pyplot as plt", None, None)?
        // );
        // println!("result {:?}", py.run("plt.plot(10,20)", None, None)?);
        // println!("result {:?}", py.run("plt.show()", None, None)?);

        // println!("call import plt");
        // let plt = py.import("matplotlib.pyplot")?;
        // println!("result {:?}", plt);
        //
        // println!("call plot");
        // println!(
        //     "result {:?}",
        //     plt.call_method1("plot", ([10, 100], [20, 200]))?
        // );
        //
        // println!("call show");
        // println!("result {:?}", plt.call_method0("show")?);

        Ok(())
    })
    .unwrap();
    Ok(())
}