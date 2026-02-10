package org.tiktzuki.pattern.state.impl;

import lombok.extern.slf4j.Slf4j;
import org.tiktzuki.pattern.state.Input;

@Slf4j
public class DuckingState extends OnGroundState {
    @Override
    public void handleInput(Input input) {
        if (input == Input.RELEASE_DOWN) {
            log.info("STAND UP");
        } else {
            super.handleInput(input);
        }
    }
}
