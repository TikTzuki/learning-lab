package org.tiktzuki.pattern.state;

public class Main {
    public static void main(String[] args) {
        Heroine heroine = new Heroine();
        heroine.handleInput(Input.PRESS_SPACE);
        heroine.handleInput(Input.PRESS_DOWN);
    }
}
