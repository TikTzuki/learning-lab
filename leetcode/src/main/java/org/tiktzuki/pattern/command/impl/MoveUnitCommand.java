package org.tiktzuki.pattern.command.impl;

import org.tiktzuki.pattern.command.GameActor;
import org.tiktzuki.pattern.command.OneTimeUseCommand;

public class MoveUnitCommand implements OneTimeUseCommand {
    int x;
    int y;
    int xBefore;
    int yBefore;
    GameActor actor;

    public MoveUnitCommand(GameActor actor, int x, int y) {
        this.actor = actor;
        this.x = x;
        this.y = y;
    }

    @Override
    public void execute() {
        xBefore = actor.getX();
        yBefore = actor.getY();
        actor.move(x, y);
    }

    @Override
    public void undo() {
        actor.move(xBefore, yBefore);
    }
}
