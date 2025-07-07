#!/usr/bin/env python
# -*- coding: utf-8 -*-
from setuptools import setup, find_packages


setup(
    name="plonk",
    version="0.0.1",
    description="Python implementation for PLONK",
    python_requires=">=3.6,<4",
    install_requires=["py_ecc==1.4.0", "ethsnarks==0.0.1", "scipy>=1.4.1",],
    extras_require={
        "test": ["pytest>=5,<6", "nbval>=0.9.5"],
        "lint": ["black>=19.10b0"],
        "tutorial": ["jupyter>=1.0.0"],
    },
    packages=find_packages(exclude=["tests", "*.ipynb"]),
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Natural Language :: English",
        "Programming Language :: Python :: 3.6",
    ],
)
