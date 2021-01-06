# ruPyInven
python extention written by rust for inventory count

# install
 cd project root
 
 pip install maturin
 
 maturin build

or

maturin build --interpreter python

or

maturin develop
install to virtual env

or

cargo build --release
then change target/release *.dll to *.pyd and import directly.

The wheel file will be generated in the ./target/wheel
