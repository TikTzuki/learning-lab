package org.tiktzuki.pattern.observer;

import lombok.extern.slf4j.Slf4j;

import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.SubmissionPublisher;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicLong;
import java.util.stream.Stream;

@Slf4j
public class Main {
    public static void simpleCase() throws InterruptedException {
        SubmissionPublisher<Message> publisher = new Entity();

        final Achievement achievements = new Achievement();
        publisher.subscribe(achievements);

        Stream.iterate(1, n -> n + 1)
                .parallel()
                .map(n -> new Message("Message " + n))
                .limit(10)
                .toList()
                .forEach(publisher::submit);

        try {
            log.info("Wait 1_000 millis....");
            Thread.sleep(1_000);
        } catch (InterruptedException exception) {
            log.error("Error ", exception);
            throw exception;
        }

        log.info("Let's close the publisher");
        publisher.close();
    }

    public static void exceptionCase() throws InterruptedException {
        try (ScheduledExecutorService executor = Executors.newScheduledThreadPool(1)) {
            SubmissionPublisher<Message> publisher = new Entity();
            Achievement achievements = new Achievement();
            publisher.subscribe(achievements);

            AtomicLong frameNumber = new AtomicLong();

            executor.scheduleWithFixedDelay(() -> publisher.offer(
                    new Message(String.valueOf(frameNumber.getAndIncrement())),
                    (subscriber, message) -> {
                        subscriber.onError(new RuntimeException("Frame# %s dropped because of backpressure".formatted(message.getContent())));
                        return true;
                    }), 0, 1, TimeUnit.MILLISECONDS);
            achievements.onError(new RuntimeException("Error"));

            Thread.sleep(1_000);

            log.info("Let's close the publisher");
            publisher.close();
        }
    }

    public static void main(String[] args) throws InterruptedException {
        exceptionCase();
    }
}
