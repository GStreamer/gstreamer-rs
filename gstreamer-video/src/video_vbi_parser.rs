// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, VideoFormat};
use glib::translate::*;

use std::fmt;

use crate::video_vbi::line_buffer_len;
use crate::{VideoAncillaryDID, VideoAncillaryDID16, VideoVBIError};

glib::wrapper! {
    #[doc(alias = "GstVideoAncillary")]
    pub struct VideoAncillary(BoxedInline<ffi::GstVideoAncillary>);
}

impl VideoAncillary {
    pub fn did_u8(&self) -> u8 {
        self.inner.DID
    }

    pub fn did(&self) -> VideoAncillaryDID {
        unsafe { VideoAncillaryDID::from_glib(self.inner.DID as ffi::GstVideoAncillaryDID) }
    }

    pub fn sdid_block_number(&self) -> u8 {
        self.inner.SDID_block_number
    }

    pub fn did16(&self) -> VideoAncillaryDID16 {
        unsafe {
            VideoAncillaryDID16::from_glib(
                (((self.inner.DID as u16) << 8) + self.inner.SDID_block_number as u16)
                    as ffi::GstVideoAncillaryDID16,
            )
        }
    }

    pub fn len(&self) -> usize {
        self.inner.data_count as usize
    }

    pub fn is_empty(&self) -> bool {
        self.inner.data_count == 0
    }

    pub fn data(&self) -> &[u8] {
        &self.inner.data[0..(self.inner.data_count as usize)]
    }
}

impl fmt::Debug for VideoAncillary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VideoAncillary")
            .field("did", &self.did())
            .field("sdid_block_number", &self.sdid_block_number())
            .field("did16", &self.did16())
            .field("data_count", &self.inner.data_count)
            .finish()
    }
}

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct VideoVBIParserInner(Boxed<ffi::GstVideoVBIParser>);

    match fn {
        copy => |ptr| ffi::gst_video_vbi_parser_copy(ptr),
        free => |ptr| ffi::gst_video_vbi_parser_free(ptr),
        type_ => || ffi::gst_video_vbi_parser_get_type(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoVBIParser {
    inner: VideoVBIParserInner,
    line_buffer_len: usize,
}

impl VideoVBIParser {
    #[doc(alias = "gst_video_vbi_parser_new")]
    pub fn try_new(format: VideoFormat, pixel_width: u32) -> Result<VideoVBIParser, VideoVBIError> {
        skip_assert_initialized!();
        let res: Option<VideoVBIParserInner> = unsafe {
            from_glib_full(ffi::gst_video_vbi_parser_new(
                format.into_glib(),
                pixel_width,
            ))
        };

        Ok(VideoVBIParser {
            inner: res.ok_or(VideoVBIError::Unsupported)?,
            line_buffer_len: line_buffer_len(format, pixel_width),
        })
    }

    // rustdoc-stripper-ignore-next
    /// Returns the buffer length needed to store the line.
    pub fn line_buffer_len(&self) -> usize {
        self.line_buffer_len
    }

    #[doc(alias = "gst_video_vbi_parser_add_line")]
    pub fn add_line(&mut self, data: &[u8]) -> Result<(), VideoVBIError> {
        if data.len() < self.line_buffer_len {
            return Err(VideoVBIError::InsufficientLineBufLen {
                found: data.len(),
                expected: self.line_buffer_len,
            });
        }
        unsafe {
            let data = data.as_ptr();
            ffi::gst_video_vbi_parser_add_line(self.inner.to_glib_none_mut().0, data);
        }

        Ok(())
    }

    pub fn iter(&mut self) -> AncillaryIter {
        AncillaryIter { parser: self }
    }

    #[doc(alias = "gst_video_vbi_parser_get_ancillary")]
    pub fn next_ancillary(&mut self) -> Option<Result<VideoAncillary, VideoVBIError>> {
        unsafe {
            let mut video_anc = std::mem::MaybeUninit::uninit();
            let res = ffi::gst_video_vbi_parser_get_ancillary(
                self.inner.to_glib_none_mut().0,
                video_anc.as_mut_ptr(),
            );

            match res {
                ffi::GST_VIDEO_VBI_PARSER_RESULT_OK => Some(Ok(VideoAncillary {
                    inner: video_anc.assume_init(),
                })),
                ffi::GST_VIDEO_VBI_PARSER_RESULT_DONE => None,
                ffi::GST_VIDEO_VBI_PARSER_RESULT_ERROR => Some(Err(VideoVBIError::NotEnoughData)),
                _ => unreachable!(),
            }
        }
    }
}

unsafe impl Send for VideoVBIParser {}
unsafe impl Sync for VideoVBIParser {}

impl<'a> TryFrom<&'a crate::VideoInfo> for VideoVBIParser {
    type Error = VideoVBIError;

    fn try_from(info: &'a crate::VideoInfo) -> Result<VideoVBIParser, VideoVBIError> {
        skip_assert_initialized!();
        VideoVBIParser::try_new(info.format(), info.width())
    }
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct AncillaryIter<'a> {
    parser: &'a mut VideoVBIParser,
}

impl Iterator for AncillaryIter<'_> {
    type Item = Result<VideoAncillary, VideoVBIError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parser.next_ancillary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VBI_HD_MIN_PIXEL_WIDTH;

    fn init_line_buf(parser: &VideoVBIParser, anc_buf: &[u8]) -> Vec<u8> {
        skip_assert_initialized!();
        let mut line_buf = vec![0; parser.line_buffer_len()];
        line_buf[0..anc_buf.len()].copy_from_slice(anc_buf);
        line_buf
    }

    #[test]
    fn cea608_component() {
        let mut parser =
            VideoVBIParser::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        let line_buf = init_line_buf(
            &parser,
            &[
                0x00, 0x00, 0x00, 0x00, 0xff, 0x03, 0xf0, 0x3f, 0x00, 0x84, 0x05, 0x00, 0x02, 0x01,
                0x30, 0x20, 0x00, 0x00, 0x06, 0x00, 0x94, 0x01, 0xc0, 0x12, 0x00, 0x98, 0x0a, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        );
        parser.add_line(&line_buf).unwrap();

        let video_anc = parser.next_ancillary().unwrap().unwrap();
        assert_eq!(video_anc.did16(), VideoAncillaryDID16::S334Eia608);
        assert_eq!(video_anc.data(), [0x80, 0x94, 0x2c]);

        assert!(parser.next_ancillary().is_none());
    }

    #[test]
    fn cea608_composite() {
        let mut parser =
            VideoVBIParser::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        let line_buf = init_line_buf(
            &parser,
            &[
                0x00, 0xf0, 0x0f, 0x00, 0x61, 0x01, 0x20, 0x10, 0x00, 0x0c, 0x08, 0x00, 0x15, 0x01,
                0x40, 0x19, 0x00, 0xb0, 0x04, 0x00, 0x3b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        );
        parser.add_line(&line_buf).unwrap();

        let video_anc = parser.next_ancillary().unwrap().unwrap();
        assert_eq!(video_anc.did16(), VideoAncillaryDID16::S334Eia608);
        assert_eq!(video_anc.data(), [0x15, 0x94, 0x2c]);

        assert!(parser.next_ancillary().is_none());
    }

    #[test]
    fn cea608_can_not_parse() {
        let mut parser =
            VideoVBIParser::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        let line_buf = init_line_buf(&parser, &[0x00, 0xf0, 0x0f, 0x00, 0x61, 0x01, 0x20, 0x10]);
        parser.add_line(&line_buf).unwrap();

        assert!(parser.next_ancillary().is_none());
    }

    #[test]
    fn cea608_insufficient_line_buf_len() {
        let mut parser =
            VideoVBIParser::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        let line_buf = vec![0; 10];

        assert_eq!(
            parser.add_line(&line_buf).unwrap_err(),
            VideoVBIError::InsufficientLineBufLen {
                found: 10,
                expected: parser.line_buffer_len()
            },
        );
    }

    #[test]
    fn cea708_component() {
        let mut parser =
            VideoVBIParser::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        let line_buf = init_line_buf(
            &parser,
            &[
                0x00, 0x00, 0x00, 0x00, 0xff, 0x03, 0xf0, 0x3f, 0x00, 0x84, 0x05, 0x00, 0x01, 0x01,
                0x50, 0x25, 0x00, 0x58, 0x0a, 0x00, 0x69, 0x02, 0x50, 0x25, 0x00, 0xfc, 0x08, 0x00,
                0x43, 0x01, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0x72, 0x02, 0x80, 0x1f, 0x00, 0xf0,
                0x0b, 0x00, 0x94, 0x01, 0xc0, 0x12, 0x00, 0xe4, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00,
                0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8,
                0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00,
                0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8,
                0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00,
                0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8,
                0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20,
                0x00, 0xe8, 0x0b, 0x00, 0x00, 0x02, 0x00, 0x20, 0x00, 0xd0, 0x09, 0x00, 0x00, 0x02,
                0x00, 0x20, 0x00, 0x6c, 0x08, 0x00, 0xb7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        );
        parser.add_line(&line_buf).unwrap();

        let video_anc = parser.next_ancillary().unwrap().unwrap();
        assert_eq!(video_anc.did16(), VideoAncillaryDID16::S334Eia708);
        assert_eq!(
            video_anc.data(),
            [
                0x96, 0x69, 0x55, 0x3f, 0x43, 0x00, 0x00, 0x72, 0xf8, 0xfc, 0x94, 0x2c, 0xf9, 0x00,
                0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0x74, 0x00, 0x00,
                0x1b,
            ]
        );

        assert!(parser.next_ancillary().is_none());
    }

    #[test]
    fn cea608_and_cea708_component() {
        let mut parser =
            VideoVBIParser::try_new(VideoFormat::V210, VBI_HD_MIN_PIXEL_WIDTH).unwrap();
        let mut line_buf = vec![0; parser.line_buffer_len()];
        let anc_buf = [
            0x00, 0x00, 0x00, 0x00, 0xff, 0x03, 0xf0, 0x3f, 0x00, 0x84, 0x05, 0x00, 0x02, 0x01,
            0x30, 0x20, 0x00, 0x00, 0x06, 0x00, 0x94, 0x01, 0xc0, 0x12, 0x00, 0x98, 0x0a, 0x00,
            0x00, 0x00, 0xf0, 0x3f, 0x00, 0xfc, 0x0f, 0x00, 0x61, 0x01, 0x10, 0x10, 0x00, 0x54,
            0x09, 0x00, 0x96, 0x02, 0x90, 0x26, 0x00, 0x54, 0x09, 0x00, 0x3f, 0x02, 0x30, 0x14,
            0x00, 0x00, 0x08, 0x00, 0x00, 0x02, 0x20, 0x27, 0x00, 0xe0, 0x07, 0x00, 0xfc, 0x02,
            0x40, 0x19, 0x00, 0xb0, 0x04, 0x00, 0xf9, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
            0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00,
            0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20,
            0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02,
            0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
            0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00,
            0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20,
            0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02,
            0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
            0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00,
            0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20,
            0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02,
            0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00,
            0xfa, 0x02, 0x00, 0x20, 0x00, 0x00, 0x08, 0x00, 0x74, 0x02, 0x00, 0x20, 0x00, 0x00,
            0x08, 0x00, 0x1b, 0x02, 0x70, 0x2b,
        ];
        line_buf[0..anc_buf.len()].copy_from_slice(&anc_buf);
        parser.add_line(&line_buf).unwrap();

        let mut anc_iter = parser.iter();

        let video_anc = anc_iter.next().unwrap().unwrap();
        assert_eq!(video_anc.did16(), VideoAncillaryDID16::S334Eia608);
        assert_eq!(video_anc.data(), [0x80, 0x94, 0x2c]);

        let video_anc = anc_iter.next().unwrap().unwrap();
        assert_eq!(video_anc.did16(), VideoAncillaryDID16::S334Eia708);
        assert_eq!(
            video_anc.data(),
            [
                0x96, 0x69, 0x55, 0x3f, 0x43, 0x00, 0x00, 0x72, 0xf8, 0xfc, 0x94, 0x2c, 0xf9, 0x00,
                0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
                0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00,
                0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa,
                0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0x74, 0x00, 0x00,
                0x1b,
            ]
        );

        assert!(anc_iter.next().is_none());
    }
}
