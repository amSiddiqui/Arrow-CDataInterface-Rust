"""
This script demonstrates how to use a Rust-compiled shared library in Python via PyArrow and CFFI.
It loads the 'export_int32_data' function from the Rust library, which populates an ArrowArray with int32 data.
The script then imports this data into a PyArrow Array and prints its contents.
"""

import pyarrow as pa
from pyarrow.cffi import ffi


def main():
    ffi.cdef("void export_int32_data(struct ArrowArray*);")
    lib = ffi.dlopen("./target/release/libarrow_exporter.dylib")
    c_arr = ffi.new("struct ArrowArray*")
    c_ptr = int(ffi.cast("uintptr_t", c_arr))
    lib.export_int32_data(c_arr)
    arrnew = pa.Array._import_from_c(c_ptr, pa.int32())
    print(arrnew)
    del arrnew


if __name__ == "__main__":
    main()
