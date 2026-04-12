use dioxus::prelude::*;

#[component]
pub(super) fn WindowResizeZones() -> Element {
    use dioxus::desktop::tao::window::ResizeDirection;

    macro_rules! zone {
        ($style:expr, $dir:expr) => {{
            let dir = $dir;
            rsx! {
                div {
                    style: $style,
                    onmousedown: move |e| {
                        e.stop_propagation();
                        let _ = dioxus::desktop::window().window.drag_resize_window(dir);
                    },
                }
            }
        }};
    }

    rsx! {
        {zone!("position:fixed;top:0;left:16px;right:16px;height:10px;cursor:n-resize;z-index:9999;",  ResizeDirection::North)}
        {zone!("position:fixed;bottom:0;left:16px;right:16px;height:10px;cursor:s-resize;z-index:9999;", ResizeDirection::South)}
        {zone!("position:fixed;left:0;top:16px;bottom:16px;width:10px;cursor:w-resize;z-index:9999;",  ResizeDirection::West)}
        {zone!("position:fixed;right:0;top:16px;bottom:16px;width:10px;cursor:e-resize;z-index:9999;",  ResizeDirection::East)}
        {zone!("position:fixed;top:0;left:0;width:16px;height:16px;cursor:nw-resize;z-index:10000;",  ResizeDirection::NorthWest)}
        {zone!("position:fixed;top:0;right:0;width:16px;height:16px;cursor:ne-resize;z-index:10000;", ResizeDirection::NorthEast)}
        {zone!("position:fixed;bottom:0;left:0;width:16px;height:16px;cursor:sw-resize;z-index:10000;", ResizeDirection::SouthWest)}
        {zone!("position:fixed;bottom:0;right:0;width:16px;height:16px;cursor:se-resize;z-index:10000;", ResizeDirection::SouthEast)}
    }
}
