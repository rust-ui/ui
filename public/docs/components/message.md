+++
title = "Message"
description = "Displays a message in a conversation, with optional avatar, header, footer, and alignment."
tags = []
is_new = true
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++


<StaticMessage />




## Installation

<StaticInstallMessage />




## Usage

```rust
use crate::components::ui::message::{Message, MessageAvatar, MessageContent, MessageHeader, MessageFooter, MessageGroup};
```

```rust
<Message>
    <MessageAvatar>
        <Avatar>
            <AvatarImage attr:src="/avatars/02.png" attr:alt="@avatar" />
            <AvatarFallback>"CN"</AvatarFallback>
        </Avatar>
    </MessageAvatar>
    <MessageContent>
        <Bubble>
            <BubbleContent>"How can I help you today?"</BubbleContent>
        </Bubble>
    </MessageContent>
</Message>
```

## Composition

```
Message
├── MessageAvatar
└── MessageContent
    ├── MessageHeader
    ├── Bubble
    └── MessageFooter
```

Use `MessageGroup` to stack consecutive messages from the same sender:

```
MessageGroup
├── Message
└── Message
```

## Examples

### Avatar

Use `MessageAvatar` to render an avatar next to the message. Set `align=MessageAlign::End` on the message to align the avatar to the end.

<StaticMessageAvatar />

### Group

Use `MessageGroup` to stack consecutive messages from the same sender. Render an empty `MessageAvatar` on the earlier messages to keep them aligned.

<StaticMessageGroup />

### Header and Footer

Use `MessageHeader` for a sender name and `MessageFooter` for metadata such as a delivery or read status.

<StaticMessageHeaderFooter />

### Actions

Place message-level actions in `MessageFooter`, such as copy, retry, or feedback buttons.

<StaticMessageActions />

### Attachment

<StaticMessageAttachment />

## See Also

- [Bubble](/docs/components/bubble)
- [Marker](/docs/components/marker)
- [Avatar](/docs/components/avatar)
