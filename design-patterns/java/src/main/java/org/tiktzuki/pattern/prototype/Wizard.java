package org.tiktzuki.pattern.prototype;

public class Wizard extends Monster {
    int health;
    int speed;
    String[] spells;

    @Override
    public Monster copy() {
        var it = new Wizard();
        it.health = 0;
        it.speed = 100;
        return it;
    }

}
