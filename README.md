# Notes for Rust Library

## Building

Run build.sh.

## create fat lib for ios

- `find target -name "*.a" | grep ios | grep rust | xargs lipo -create -output "libhello_rust-ios.a"`

- "cp `find target -name "*.a" | grep linux | grep rust` libhello_rust-android.a"

- `cp libhello_rust-android.a ../android/hello-jni/jni/libhello_rust-android.a`

- `cp libhello_rust-ios.a ../ios/libhello_rust-ios.a`

# API

Actions are processed in Rust. Everything is always async, and at the JS -> Rust interface layer we'll always return a promise.

## Profiles

Get Profiles stored locally. Each profile may be associated with multiple git repositories.

```json
{
  "action": "profiles_get_all",
  "parameters": { "username": "jdoe" }
}
```

## Feed

Get a feed

## Git Host

Check username availability.

```json
{
  "action": "githost_check_username_availability",
  "parameters": { "username": "jdoe" }
}
```

Register User

```json
{
  "action": "githost_register_user",
  "parameters": {
    "name": "John Doe",
    "email": "jdoe@agilehead.com",
    "username": "jdoe",
    "password": "secret"
  }
}
```

## Sync

Add Sync

```json
{
  "action": "sync_add",
  "parameters": { "url": "https://github.com/jeswin/gitstadata" }
}
```

Remove Sync

```json
{
  "action": "sync_remove",
  "parameters": { "url": "https://github.com/jeswin/gitstadata" }
}
```

## Query

Sqlite Queries

```json
{
  "action": "sqlite_query",
  "parameters": {
    "query": "SELECT * FROM comments WHERE username = @username",
    "parameters": {
      "username": "jeswin"
    }
  }
}
```
