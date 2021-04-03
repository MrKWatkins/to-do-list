CREATE TABLE [tasks]
(
    [id] INTEGER NOT NULL,
    [name] TEXT NOT NULL,
    [description] TEXT NULL,
    [deadline] DATE NULL,
    [completed] TIMESTAMP NULL,

    CONSTRAINT [pk_id] PRIMARY KEY ([id])
);

CREATE INDEX [ix_completed] ON [tasks] ([completed]);