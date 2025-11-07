#!/bin/bash

echo "executing run_benches.sh"
cargo run --release &> out_rust_oneshot.txt
cargo bench &> out_rust_criterion.txt
python3 py_impl.py &> out_python_oneshot.txt
pytest test_py.py --benchmark-only &> out_python_pytest.txt
