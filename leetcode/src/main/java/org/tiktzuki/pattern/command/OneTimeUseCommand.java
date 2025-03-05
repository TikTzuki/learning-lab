package org.tiktzuki.pattern.command;

public interface OneTimeUseCommand {
    void execute();

    void undo();
}
