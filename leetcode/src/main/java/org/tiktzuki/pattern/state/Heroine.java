package org.tiktzuki.pattern.state;

import lombok.Data;

@Data
public class Heroine {
    HeroineState movingState;
    HeroineState equipment;

    public void handleInput(Input input) {
        movingState.handleInput(input);
        equipment.handleInput(input);
    }
}
