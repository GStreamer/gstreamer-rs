// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use crate::Stream;

impl Stream {
    pub fn debug(&self) -> Debug {
        Debug(self)
    }
}

pub struct Debug<'a>(&'a Stream);

impl fmt::Debug for Debug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Stream")
            .field("stream_id", &self.0.stream_id())
            .field("stream_type", &self.0.stream_type())
            .field("stream_flags", &self.0.stream_flags())
            .field("caps", &self.0.caps())
            .field("tags", &self.0.tags())
            .finish()
    }
}
