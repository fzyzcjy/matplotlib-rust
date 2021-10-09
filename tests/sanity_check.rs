use std::time::Duration;

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

// ignore the test, only for sanity
#[ignore]
#[test]
fn matplotlib_sanity_check() -> Result<()> {
    Python::with_gil(|py| -> Result<()> {
        println!("call import matplotlib");
        let matplotlib = py.import("matplotlib")?;
        println!("result {:?}", matplotlib);

        println!("call get_backend");
        println!("result {:?}", matplotlib.call_method0("get_backend")?);

        println!("call version");
        println!("result {:?}", matplotlib.getattr("__version__")?);

        println!(
            "result {:?}",
            py.run(
                "\
import matplotlib.pyplot as plt;
plt.ion();
print('after import');
print(plt.plot);
print([10,20], [100,110]);
plt.plot([10,20], [100,110]);
print('after plot');
plt.show();
print('after show');
                    ",
                None,
                None,
            )?
        );

        std::thread::sleep(Duration::from_millis(500));
        println!(
            "result {:?}",
            py.run(
                "\
print('lets plott again');
plt.plot([10,50], [100,110]);
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
        // println!("call ion");
        // println!("result {:?}", plt.call_method0("ion")?);
        //
        // println!("call plot");
        // println!(
        //     "result {:?}",
        //     plt.call_method1("plot", ([10, 20], [0, 100]))?
        // );
        //
        // println!("call show");
        // println!("result {:?}", plt.call_method0("show")?);
        //
        // for i in 0..3 {
        //     std::thread::sleep(Duration::from_millis(500));
        //
        //     println!("call plot again");
        //     println!(
        //         "result {:?}",
        //         plt.call_method1("plot", ([10, (i + 2) * 20], [0, 100]))?
        //     );
        //
        //     println!("call show");
        //     println!("result {:?}", plt.call_method0("show")?);
        // }

        Ok(())
    })
    .unwrap();
    Ok(())
}
