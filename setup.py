from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="frcLogLens",
    version="0.11",
    rust_extensions=[RustExtension("rs_frc_log_lens", binding=Binding.PyO3)],
    packages=["frc_log_lens"],
    package_dir={"": "src/py"},
    zip_safe=False,
    install_requires=['robotpy-wpiutil >= 2023'],
)
