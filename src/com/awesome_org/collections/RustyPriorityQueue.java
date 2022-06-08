package com.awesome_org.collections;

import java.io.IOException;

// generate JNI function names via `javac -h . ./src/com/awesome_org/collections/RustyPriorityQueue.java`
// compile via `javac ./src/com/awesome_org/collections/RustyPriorityQueue.java`
// run via `java -cp ./src/ -Djava.library.path=./rust/target/debug/ com.awesome_org.collections.RustyPriorityQueue`
public class RustyPriorityQueue {
    static {
        System.loadLibrary("pqueue_jni");
    }

    private long objPtr;

    private static native long pq_create(byte[] elements);

    private native void pq_free(long objPtr);

    private native void pq_push(long objPtr, byte element);

    private native byte pq_pop(long objPtr);

    public RustyPriorityQueue(byte[] elements) {
        this.objPtr = pq_create(elements);
    }

    public void destroy() {
        if (this.objPtr == 0)
            throw new IllegalStateException("Access of native queue after destruction!");

        pq_free(this.objPtr);

        this.objPtr = 0;
    }

    public void push(byte element) {
        if (this.objPtr == 0)
            throw new IllegalStateException("Access of native queue after destruction!");

        pq_push(this.objPtr, element);
    }

    public byte pop() {
        if (this.objPtr == 0)
            throw new IllegalStateException("Access of native queue after destruction!");

        return pq_pop(this.objPtr);
    }

    public static void main(String... args) {
    /*
        System.out.println("Wait for connecting debugger. Press any key to continue!");
        try {
            System.in.read();
        } catch (IOException e) {
            // ignore
        }
    */

        var elements = new byte[]{4, 2, 5, 1, 2};

        var pq = new RustyPriorityQueue(elements);
        pq.push((byte) 6);

        for (int i = 0; i < 10; i++) {
            try {
                byte result = pq.pop();
                System.out.println("pop() = " + result);
            } catch (Exception e) {
                System.out.println("pop() = Nothing");
            }
        }
        pq.destroy();
        pq = null;
    }

}