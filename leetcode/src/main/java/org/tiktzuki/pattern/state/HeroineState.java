package org.tiktzuki.pattern.state;

public interface HeroineState {
    void enter(Heroine heroine);

    void handleInput(Input input);
}
