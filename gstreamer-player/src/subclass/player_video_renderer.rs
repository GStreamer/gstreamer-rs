// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{ffi, Player, PlayerVideoRenderer};

pub trait PlayerVideoRendererImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<PlayerVideoRenderer>>
{
    fn create_video_sink(&self, player: &Player) -> gst::Element;
}

unsafe impl<T: PlayerVideoRendererImpl> IsImplementable<T> for PlayerVideoRenderer {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.create_video_sink = Some(video_renderer_create_video_sink::<T>);
    }
}

pub trait PlayerVideoRendererImplExt: PlayerVideoRendererImpl {
    fn parent_create_video_sink(&self, player: &Player) -> gst::Element {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<PlayerVideoRenderer>()
                as *const ffi::GstPlayerVideoRendererInterface;

            let func = (*parent_iface)
                .create_video_sink
                .expect("no parent \"create_video_sink\" implementation");
            let ret = func(
                self.obj()
                    .unsafe_cast_ref::<PlayerVideoRenderer>()
                    .to_glib_none()
                    .0,
                player.to_glib_none().0,
            );
            from_glib_none(ret)
        }
    }
}

impl<T: PlayerVideoRendererImpl> PlayerVideoRendererImplExt for T {}

unsafe extern "C" fn video_renderer_create_video_sink<T: PlayerVideoRendererImpl>(
    video_renderer: *mut ffi::GstPlayerVideoRenderer,
    player: *mut ffi::GstPlayer,
) -> *mut gst::ffi::GstElement {
    static VIDEO_SINK_QUARK: std::sync::OnceLock<glib::Quark> = std::sync::OnceLock::new();

    let video_sink_quark =
        VIDEO_SINK_QUARK.get_or_init(|| glib::Quark::from_str("gstreamer-rs-player-video-sink"));

    let instance = &*(video_renderer as *mut T::Instance);
    let imp = instance.imp();

    let sink = imp.create_video_sink(&from_glib_borrow::<_, Player>(player));

    let sink_ptr: *mut gst::ffi::GstElement = sink.to_glib_none().0;

    let old_sink_ptr = glib::gobject_ffi::g_object_get_qdata(
        video_renderer as *mut _,
        video_sink_quark.into_glib(),
    ) as *mut gst::ffi::GstElement;
    if !old_sink_ptr.is_null() && old_sink_ptr != sink_ptr {
        panic!("Video sink must not change");
    }

    unsafe extern "C" fn unref(ptr: glib::ffi::gpointer) {
        glib::gobject_ffi::g_object_unref(ptr as *mut _);
    }

    glib::gobject_ffi::g_object_set_qdata_full(
        video_renderer as *mut _,
        video_sink_quark.into_glib(),
        glib::gobject_ffi::g_object_ref(sink_ptr as *mut _) as *mut _,
        Some(unref),
    );

    sink_ptr
}
