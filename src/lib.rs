use jni::objects::ReleaseMode::NoCopyBack;
use jni::objects::{JClass, JObject};
use jni::sys::{jbyte, jbyteArray, jlong};
use jni::JNIEnv;
use jni_fn::jni_fn;

pub struct PQueueI8(std::collections::BinaryHeap<i8>);

#[jni_fn("com.awesome_org.collections.RustyPriorityQueue")]
pub fn pq_create(
    env: JNIEnv,
    _class: JClass, // because it's a static JNI method
    j_elements: jbyteArray,
) -> jlong {
    let elements_java_array = env
        // .get_byte_array_elements(j_elements, NoCopyBack) // use this if you prefer a memory copy and not blocking Java GC.
        .get_primitive_array_critical(j_elements, NoCopyBack)
        .expect("couldn't get java primitive array");
    let elements_java_array_ptr = elements_java_array.as_ptr().cast::<i8>();

    let array_length: usize = elements_java_array
        .size()
        .expect("couldn't get array size")
        .try_into()
        .expect("couldn't convert to usize");

    // original example code - start
    let mut pq = PQueueI8(std::collections::BinaryHeap::new());

    // Dereferences `elements` without checking whether it's NULL.
    let elements: &[i8] = unsafe { std::slice::from_raw_parts(elements_java_array_ptr, array_length) };
    elements.into_iter().for_each(|item| pq.0.push(*item));

    let pq_ptr = Box::into_raw(Box::new(pq));
    // original example code - end

    pq_ptr as jlong
}

#[jni_fn("com.awesome_org.collections.RustyPriorityQueue")]
pub fn pq_push(_env: JNIEnv, _obj: JObject, native_object_ptr: jlong, element: jbyte) {
    let pq = unsafe { &mut *(native_object_ptr as *mut PQueueI8) };
    pq.0.push(element);
}

#[jni_fn("com.awesome_org.collections.RustyPriorityQueue")]
pub fn pq_pop(env: JNIEnv, _obj: JObject, native_object_ptr: jlong) -> jbyte {
    let pq = unsafe { &mut *(native_object_ptr as *mut PQueueI8) };
    if let Some(val) = pq.0.pop() {
        return val;
    } else {
        let ex_class = env
            .find_class("java/lang/ArrayIndexOutOfBoundsException")
            .expect("couldn't find exception class");
        env.throw_new(ex_class, "Queue is empty.")
            .expect("couldn't throw exception");

        return i8::MIN; // dummy value to satisfy the compiler. It won't be consumed on the Java side because of the exception above.
    }
}

#[jni_fn("com.awesome_org.collections.RustyPriorityQueue")]
pub fn pq_free(_env: JNIEnv, _obj: JObject, native_object_ptr: jlong) {
    unsafe {
        // The Box will be destructed when going out-of-scope, still it doesn't hurt to call drop() explicitly.
        drop(Box::from_raw(native_object_ptr as *mut PQueueI8));
    }
}
