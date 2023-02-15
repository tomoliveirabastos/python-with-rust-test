import ctypes
import json

def sum():
       lib = ctypes.cdll.LoadLibrary('./target/release/librust_with_python_test.so')
       r = lib.add(1, 2)
       print(r)

def say_my_name():
       lib = ctypes.cdll.LoadLibrary('./target/release/librust_with_python_test.so')
       sayMyName = lib.say_my_name
       sayMyName.restype = ctypes.c_void_p
       out = sayMyName("Tom".encode("utf-8"))
       pointer = ctypes.string_at(out)
       r = pointer.decode("utf-8")
       print(r)


def send_json():
       lib = ctypes.cdll.LoadLibrary('./target/release/librust_with_python_test.so')
       pessoa = {
              "nome": "Tom",
              "idade": 20
       }

       a = lib.handle_json
       a.restype = ctypes.c_void_p
       a(json.dumps(pessoa).encode("utf-8"))


def get_json():
       lib = ctypes.cdll.LoadLibrary('./target/release/librust_with_python_test.so')
       json = lib.pessoa_to_json
       json.restype = ctypes.c_void_p
       r = json(
              "Tom Oliveira".encode("utf-8"),
              20
       )
       pointer = ctypes.string_at(r)
       print(pointer.decode("utf-8"))


def main():
       sum()
       say_my_name()
       send_json()
       get_json()

main()