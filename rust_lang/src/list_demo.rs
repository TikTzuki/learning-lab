// methods on.
use jni::JNIEnv;
// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JByteArray, JClass};

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_org_tiktzuki_jni_JListDemo_helloByte<'local, 'other_local_1, 'obj_ref>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JByteArray<'local>)
    -> JByteArray<'local> {
    let _input = env.convert_byte_array(&input).unwrap();

    let buf = [1; 2_000];
    let output = env.byte_array_from_slice(&buf).unwrap();

    output
}
