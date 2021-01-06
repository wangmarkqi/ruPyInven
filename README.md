# ruPyInven
python extention written by rust for inventory count

# install
## by maturin build to whl
    cd project root

    pip install maturin

    maturin build or maturin build --interpreter python

## by maturin install to virtual env

    maturin develop


## build to dynamic lib, no depending on python version
    cargo build --release

    and change target/release *.dll to *.pyd and import directly.

