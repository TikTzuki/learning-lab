package org.tiktzuki.pattern.observer;

import lombok.extern.slf4j.Slf4j;

import java.util.concurrent.Flow.Subscriber;
import java.util.concurrent.Flow.Subscription;

@Slf4j
public class Achievement implements Subscriber<Message> {
    String name = "ZZZ";
    Subscription subscription;

    @Override
    public void onSubscribe(Subscription subscription) {
        log.info("New subscription: {}", subscription);
        this.subscription = subscription;
        subscription.request(1);
    }

    @Override
    public void onNext(Message item) {
        log.info("New message: {}", item);
        subscription.request(1);
    }

    @Override
    public void onError(Throwable throwable) {
        throwable.printStackTrace();
    }

    @Override
    public void onComplete() {
        log.debug("Complete");
    }
}
