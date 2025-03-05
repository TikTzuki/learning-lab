package org.tiktzuki.pattern.state.impl;

import lombok.extern.slf4j.Slf4j;
import org.tiktzuki.pattern.state.Heroine;
import org.tiktzuki.pattern.state.HeroineState;
import org.tiktzuki.pattern.state.Input;

@Slf4j
public abstract class OnGroundState implements HeroineState {
    @Override
    public void enter(Heroine heroine) {

    }

    @Override
    public void handleInput(Input input) {
        if (input == Input.PRESS_SPACE) {
            log.info("heroine.jump()");
        }
    }
}
