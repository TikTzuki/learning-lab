package org.tiktzuki.pattern.typeobject;

import lombok.Data;

import java.util.Objects;

@Data
public class Breed {
    Breed parent;
    int health;
    String attack;

    public Breed(int health, String attack) {
        this.health = health;
        this.attack = attack;
    }

    public Breed(Breed parent) {
        if (Objects.nonNull(parent)) {
            this.parent = parent;
            this.health = parent.health;
            this.attack = parent.attack;
        }
    }

}
