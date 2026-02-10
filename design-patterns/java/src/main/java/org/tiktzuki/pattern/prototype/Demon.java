package org.tiktzuki.pattern.prototype;

public class Demon extends Monster {
    int health;
    int speed;
    String[] attacks;

    @Override
    public Monster copy() {
        var it = new Demon();
        it.health = 100;
        it.speed = 50;
        return it;
    }

}
