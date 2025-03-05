package org.tiktzuki.jni;

import lombok.extern.slf4j.Slf4j;

import java.util.List;

@Slf4j
public class JListDemo {
    private static native List<String> process(List<String> cards);

    private static native byte[] helloByte(byte[] input);

    private static native void factAndCallMeBack(int n, JListDemo factCallback);

    private static native long counterNew(JListDemo callback);

    private static native void counterIncrement(long counterPtr);
    private static native void counterDestroy(long counterPtr);
    private static native void asyncComputation(JListDemo callback);

    static {
        log.info(System.getenv("DYLD_LIBRARY_PATH"));
        System.loadLibrary("rust_lang");
    }

    public static void main(String[] args) {
//        byte[] result = JListDemo.helloByte("Hello, World!".getBytes());
//        System.out.println(new String(result, StandardCharsets.UTF_8));

//        factAndCallMeBack(5, new JListDemo());

        long counterPtr = counterNew(new JListDemo());
        for (int i = 0; i < 5; i++) {
            counterIncrement(counterPtr);
        }

        counterDestroy(counterPtr);

        System.out.println("Invoking asyncComputation (thread id = " + Thread.currentThread().threadId() + ")");
        asyncComputation(new JListDemo());
    }

    public void accept(int res) {
        System.out.println("factCallback: res = " + res);
    }

    public void counterCallback(int count) {
        System.out.println("counterCallback: count = " + count);
    }

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: thread id = " + Thread.currentThread().threadId() + ", progress = " + progress + "%");
    }
}
