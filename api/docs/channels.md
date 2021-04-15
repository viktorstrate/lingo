# Channels

## List public channels

Get public channels

> GET /api/channels

```ts
interface ListChannelsRequest {
  visibility: "public" | "private",
  paginate: Pagination
}
```

```ts
type ListChannelsResponse = [Channel]
```


## Make channel

> POST /api/channels

```ts
interface MakeChannelRequest {
  name: string,
  visibility: "public" | "private",
}
```

```ts
interface MakeChannelResponse {
  name: string,
}
```