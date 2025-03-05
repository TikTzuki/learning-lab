package org.tiktzuki.pattern.command;

public interface GameActor {
    void jump();

    void moveLeft();

    void moveRight();

    void fire();

    void stop();

    int getX();

    int getY();

    void move(int x, int y);
}
