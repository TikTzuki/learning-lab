package org.tiktzuki.pattern.command.impl;

import org.tiktzuki.pattern.command.Command;
import org.tiktzuki.pattern.command.GameActor;

public class JumpCommand implements Command {
    @Override
    public void execute(GameActor actor) {
        actor.jump();
    }
}
