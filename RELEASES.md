# `atomicow` Release Notes

## Version 1.1

- added support for `no_std` platforms: turn off `default_features` to disable the new `std` feature
- added `AtomicCow::new_owned`
- added a `From<Arc<T>> for AtomicCow<T>` implementation

## Version 1.0.0

- initial release
  - code was taken directly from `bevy_utils 0.14`, under a MIT + Apache dual license
