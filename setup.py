import os
import sys
import subprocess
from setuptools import setup
from setuptools_rust import RustExtension
from setuptools.command.test import test as TestCommand


class PyTest(TestCommand):

    def run(self):
        self.run_command("test_rust")

        subprocess.check_call(["pytest", "tests"])


setup_requires = [
    "setuptools-rust>=0.10.1", "wheel"
]

setup(
    name="mepuru",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
    ],
    packages=["mepuru"],
    rust_extensions=[
        RustExtension("mepuru.mepuru", "Cargo.toml", debug=False)
    ],
    setup_requires=setup_requires,
    install_requires=[],
    tests_require=[],
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict(test=PyTest),
)
