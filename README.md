# `tstreamer` - Token Streamer

Is a **WIP** library inspired by `gstreamer` for real-time chat applications. It is designed to be a flexible and extensible framework for building AI data processing pipelines. It has several planned backends including Anthropic, OpenAI, and local language models.

## Features

- [x] - Anthropic backend
- [ ] - OpenAI backend
- [ ] - Local language model backend

## Initial design choices:

- **Dynamic dispatch** for most things.
  - Because there are many, many, possible permutations of elements, buffers, and so on.
  - Because it allows for easy extensibility.
  - Because the app is IO bound, not CPU bound.
- **Crib from GStreamer**. Their design is a good, mature one. `Pipeline`s have `Element`s with one or more links to other `Element`s along which `Buffers` are sent.
- **Async**. GStreamer and Blender have manual thread barriers where the pipeline splits. This is a good design but much of the work in such pipelines is (frequently) CPU bound while ours is IO -- making calls to APIs and databases. Rust async is mature enough for this, including all our depenedencies.
- **Serializable**. Again, cribbing from GStreamer, the pipeline should be serializable. We do this by serializing the configuration used to create the pipeline, not the pipeline itself (because too many things can't be serialized and because it breaks object safety if we require it). A gstreamer-style string representation of the pipeline is also possible, but not a priority. It would require writing a parser for the string representation and those are very easy to get wrong. `serde` is therefore what we use and we'll likely create a UI for creating pipelines.
- **Forbid Unsafe**. Because this will be used in services exposed to the internet and because we're not doing anything that requires `unsafe`. And because it's usually faster to write safe code than unsafe code.

## Status

- It builds, but do not expect it to do much yet. The `Element` and `Buffer` traits are written and implemented for Anthropic, but not the `Pipeline` that will put them all together in a graph.
