package org.tiktzuki.pattern.typeobject;

public class Monster {
    Breed breed;
    int health;
    private static final int LOW_HEALTH = 10;

    public Monster(Breed breed) {
        this.breed = breed;
        this.health = breed.getHealth();
    }

    public String getAttack() {
        if (health < LOW_HEALTH) {
            return "The monster flails weakly.";
        }
        return breed.getAttack();
    }
}
