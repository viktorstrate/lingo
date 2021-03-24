# Structure

## Server

```ts
interface Server {
  channels: [Channel]
  users: [User]
}
```

## Channels

```ts
interface Channel {
  id: ID,
  members: [User],
  textRooms: [TextRoom],
  voiceRooms: [VoiceRoom]
}
```

## Rooms

### Text rooms

```ts
interface TextRoom {
  id: ID,
  messages: [TextMessage],
}
```

```ts
interface TextMessage {
  id: ID,
  timestamp: Timestamp,
  body: String,
  user: User
}
```

### Voice rooms
