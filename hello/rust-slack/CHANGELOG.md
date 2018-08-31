## 0.6.0

FEATURES
- Replaced `curl` with `reqwest`

BREAKING CHANGES:
- `url` (removed) and `chrono` crates are no longer re-exported

OTHER:
- Removed `log` crate
- `error-chain` updated to `0.11`

## 0.5.0

FEATURES
- Support `mrkdwn_in` field for an `Attachment`: [#17](https://github.com/frostly/rust-slack/pull/17).

BREAKING CHANGE:
- Need to move to chrono 0.4 since 0.3.1 (with serde 1.0) was yanked.

## 0.4.0

FEATURES
- Upgrade to serde 1.0 #16
 - chrono 0.3.1 is now also required (serde 1.0 support added in 0.3.1). Even though chrono
didn't do this as a breaking change it should be for semver. As a result, I'm moving this crate
to 0.4.0 to avoid any issues.
- Specify `Content-Type` as json when sending messages #15

OTHER

- Replaced `quick-error` with `error-chain`

## 0.3.0

FEATURES
 - Dependencies updated:
  - Serde upgraded to 0.9 (requires rust 1.15)
  - Chrono to 0.3
  - Curl to 0.4

## 0.2.0

DEPRECATIONS/BREAKING CHANGES:

 - `Payload`
  - `PayloadTemplate` is removed.
  - `unfurl_links` is now bool. No integer transformation is required by the slack API anymore.
 - `Attachment`
  - `AttachmentTemplate` is removed.
  - `color` is now optional.
 - `SlackError` is now `Error` and `SlackResult` is now `Result`. In addition, `ErrorKind` has been
 removed. All `Error` variants have also been renamed, removing the superfluous `Err` prefix:
  - `ErrSlackResp` -> `Slack`
  - `ErrUtf8` -> `Utf8`
  - `ErrFromHex` -> `FromHex`
  - `ErrHexColor` -> `HexColor`
  - `ErrEncoder` -> REMOVED. Replaced with `Serialize`.
  - `ErrCurl` -> `Curl`
 - `SlackLink`, `SlackText`
  - The `Display` trait is now used to format strings for sending to slack rather than `Debug`.
  The `Debug` impl is derived now.
 - `Slack::new` now returns a `Result<Slack>` as it does `Url` parsing.

FEATURES

 - `PayloadBuilder` and `AttachmentBuilder` should be used for building a `Payload` or `Attachment`
 respectively. Errors won't be returned until the final `build` function is called. At this point,
 only the first error is displayed. Subsequent errors will only appear once the first error is
 fixed.
 - `Payload`
  - Added `unfurl_media` option
 - Three character hex color codes, e.g. `#000`, are now supported.
 - Add support for sending links and text into slack, see:
 [Text with Links](README.md#text-with-links).
 - Optional fields no longer are sent in the serialized json message to slack.
 - Add `parse` option to `Payload`.
 - New `Attachment` fields have been added.

OTHER

 - `TryFrom` and `TryInto` traits have been added temporarily to this crate until they are
 formalized in rust proper. See https://github.com/rust-lang/rust/issues/33417 for details.
 - `HexColorT` trait removed. Conversions are used instead.
