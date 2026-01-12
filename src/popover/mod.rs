use std::time::Duration;

use crate::{
    class_list,
    util::{callback::BoxCallback, element::Element},
};
use leptos::{
    ev::{click, mouseenter, mouseleave, on},
    html::Div,
    prelude::*,
};
use leptos::{
    logging::{debug_log, debug_warn, error, warn},
    tachys::{html::node_ref::node_ref, renderer::dom::CssStyleDeclaration},
};
use leptos_use::{
    OnClickOutsideOptions, math::use_or, on_click_outside_with_options, use_window_scroll,
};
use web_sys::{DomRect, HtmlDivElement, MouseEvent};

// TODO: Resize observer ?
#[component]
pub fn Popover<Trigger, Content>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Action that displays the popover.
    #[prop(optional)]
    trigger_type: PopoverTriggerType,
    /// The element or component that triggers popover.
    popover_trigger: PopoverTrigger<Trigger>,
    /// Configures the position of the Popover.
    #[prop(optional)]
    preferred_pos: PopoverPosition,
    /// Wether or not to render and position the popup for a connector arrow between the popover and trigger element.
    #[prop(default = true, optional)]
    show_arrow: bool,
    /// Called when the popover becomes visible
    #[prop(optional, into)]
    on_open: Option<BoxCallback>,
    /// Called when the popover becomes invisible
    #[prop(optional, into)]
    on_close: Option<BoxCallback>,
    children: TypedChildren<Content>,
) -> impl IntoView
where
    Trigger: AddAnyAttr + IntoView + Send + 'static,
    Content: AddAnyAttr + IntoView + Send + 'static,
{
    let trigger_ref: NodeRef<Element> = NodeRef::new();
    let popover_ref: NodeRef<Div> = NodeRef::new();
    let arrow_ref: NodeRef<Div> = NodeRef::new();
    let show_popover_handle = StoredValue::new(None::<TimeoutHandle>);

    let show_by_hover = RwSignal::new(false);
    let popover_clicked_open = RwSignal::new(false);
    let _ = on_click_outside_with_options(
        trigger_ref,
        move |_| {
            popover_clicked_open.set(false);
        },
        OnClickOutsideOptions::default().ignore([popover_ref]),
    );

    let popover_visible = use_or(show_by_hover, popover_clicked_open);

    let (x, y) = use_window_scroll();
    Effect::watch(
        move || popover_visible.get(),
        move |new_visible, old, w| {
            if Some(new_visible) == old {
                return;
            }
            if let Some(on_open) = &on_open
                && *new_visible
            {
                on_open();
            } else if let Some(on_close) = &on_close
                && !*new_visible
            {
                on_close();
            }
        },
        false,
    );

    Effect::new(move || {
        let popover_visible = popover_visible.get();

        // update on scroll
        let _ = x.get();
        let _ = y.get();

        // Skip recalculate when invisible.
        if let Some(popover) = popover_ref.get()
            && let Some(trigger) = trigger_ref.get()
            && popover_visible
        {
            debug_log!("recalculating style");
            let (chosen_popover_position, abs_position) =
                find_popover_abs_position(preferred_pos, &popover, &trigger, show_arrow);
            if let Some(chosen_popover_position) = chosen_popover_position
                && let Some(HorizontalOffset::Left(x)) = abs_position.horizontal_offset
                && let Some(VerticalOffset::Top(y)) = abs_position.vertical_offset
                && let Some(arrow) = arrow_ref.get()
            {
                set_arrow_position(arrow, &popover, (x, y), chosen_popover_position);
            }

            debug_log!("Rel_pos: {abs_position:?}");
            let popover_style = (*popover).style();

            match abs_position.horizontal_offset {
                Some(HorizontalOffset::Left(px)) => {
                    set_popover_property(&popover_style, "left", format!("{px}px"));
                }
                Some(HorizontalOffset::Right(px)) => {
                    set_popover_property(&popover_style, "right", format!("{px}px"));
                }
                _ => {
                    warn!("No horizontal position provided for popover");
                }
            }
            match abs_position.vertical_offset {
                Some(VerticalOffset::Top(px)) => {
                    set_popover_property(&popover_style, "top", format!("{px}px"));
                }
                Some(VerticalOffset::Bot(px)) => {
                    set_popover_property(&popover_style, "bot", format!("{px}px"));
                }
                _ => {
                    warn!("No vertical position provided for popover");
                }
            }
        }
    });

    let on_mouse_enter = move |_| {
        if trigger_type != PopoverTriggerType::Hover {
            return;
        }
        show_popover_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
        show_by_hover.set(true);
    };
    let on_mouse_leave = move |e| {
        if trigger_type != PopoverTriggerType::Hover {
            return;
        }
        // Workaround for scrollbars otherwise closing the popup
        if let Some(popover) = popover_ref.get()
            && element_contains_pointer(&popover, e)
        {
            return;
        }
        show_popover_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle = set_timeout_with_handle(
                move || {
                    show_by_hover.set(false);
                },
                Duration::from_millis(100),
            )
            .ok();
        });
    };
    let on_click = move |_| {
        if trigger_type != PopoverTriggerType::Click {
            return;
        }
        popover_clicked_open.update(|old| *old = !*old);
    };

    let trigger_children = popover_trigger.children.into_inner()()
        .into_inner()
        .add_any_attr(on(mouseenter, on_mouse_enter))
        .add_any_attr(on(mouseleave, on_mouse_leave))
        .add_any_attr(on(click, on_click))
        .add_any_attr(node_ref(trigger_ref));

    // Incase I need to add attrs in the future
    let content_children = children.into_inner()()
        .into_inner()
        .add_any_attr(on(mouseenter, on_mouse_enter))
        .add_any_attr(on(mouseleave, on_mouse_leave));

    view! {
        <div class=class_list!(class)>
            {trigger_children}
            // Can't be hidden, because then the size is 0, bypass via opacity-0 and z-index.
            <div
                class=class_list![
                    "absolute bg-white border shadow-sm rounded-lg",
                    ("-z-[1000] opacity-0 left-0 top-0", move || !popover_visible.get()),
                    ("z-[1000]", move || popover_visible.get())
                ]
                node_ref=popover_ref
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                <div class="overflow-auto max-w-[40vw] max-h-[50vw] h-full w-full p-2">
                    {content_children}
                </div>
            </div>

            <Show when=move || show_arrow fallback=|| view!{ <> }>
                // The arrow part of the popover.
                // Both divs are angled 45 deg so it points right by default, inner white square overflow is clipped off
                <div class=class_list!(
                    // top-right-bordered transparent square
                    "absolute border-t border-r rotate-45 h-3 w-3 overflow-hidden",
                    ("-z-[1000] opacity-0 left-0 top-0", move || !popover_visible.get()),
                    ("z-[1001]", move || popover_visible.get())) node_ref=arrow_ref>

                    <div
                        // A clipped white square such that it becomes a bg between top-left, top-right and bottom-right corners.
                        class="relative w-5 h-3 -translate-y-1 rotate-45 bg-white"
                    />

                </div>
            </Show>
        </div>
    }
}

/// To test if the mouse is still on the element, like when hovering a scrollbar.
fn element_contains_pointer(popover_ref: &HtmlDivElement, e: MouseEvent) -> bool {
    let rect = (*popover_ref).get_bounding_client_rect();
    let x = e.x();
    let y = e.y();
    let rect_x_min = rect.x() as i32;
    let rect_y_min = rect.y() as i32;
    let rect_x_max = (rect.x() + rect.width()) as i32;
    let rect_y_max = (rect.y() + rect.height()) as i32;
    debug_log!(
        "{x} {y} in {} {} {} {}",
        rect.x(),
        rect.x() + rect.width(),
        rect.y(),
        rect.y() + rect.height()
    );
    return rect_x_min < x && rect_x_max > x && rect_y_min < y && rect_y_max > y;
}

fn set_popover_property(popover_style: &CssStyleDeclaration, property: &str, value: String) {
    if let Err(err) = popover_style.set_property(property, value.as_str()) {
        error!("{:?}", err);
    }
}

/// Gets bounding box of [e] wrt page origin 0,0 in the top left.
fn get_true_bb(e: &web_sys::Element) -> DomRect {
    let rect = e.get_bounding_client_rect();
    let Some(window) = web_sys::window() else {
        return rect;
    };
    if let Ok(s) = window.scroll_x() {
        rect.set_x(rect.x() + s);
    }
    if let Ok(s) = window.scroll_y() {
        rect.set_y(rect.y() + s);
    }
    rect
}

/// Represents a vertical offset of n pixels
#[derive(Debug)]
enum VerticalOffset {
    Top(u32),
    Bot(u32),
}

/// Represents a horizontal offset of n pixels
#[derive(Debug)]
enum HorizontalOffset {
    Left(u32),
    Right(u32),
}

/// Relative positioning information
#[derive(Debug)]
struct RelativePosition {
    horizontal_offset: Option<HorizontalOffset>,
    vertical_offset: Option<VerticalOffset>,
}

fn set_arrow_position(
    arrow_ref: HtmlDivElement,
    popover_ref: &web_sys::Element,
    popover_coords: (u32, u32),
    position: PopoverPosition,
) {
    let (base_x, base_y) = popover_coords;
    let base_x = base_x as f64;
    let base_y = base_y as f64;
    let arrow_style = (*arrow_ref).style();
    let popover_rect = get_true_bb(&popover_ref);
    let arrow_size = 12.0;
    let corner_offset = 12.0;
    let arrow_middle = arrow_size / 2.0;
    let popover_height = popover_rect.height();
    let popover_width = popover_rect.width();
    let horizontal_middle = popover_rect.width() / 2.0;
    let vertical_middle = popover_rect.height() / 2.0;
    let (left, top, rotation) = match position {
        PopoverPosition::Top => (
            base_x + horizontal_middle - arrow_middle,
            base_y - arrow_middle + popover_height,
            "135deg",
        ),
        PopoverPosition::Bottom => (
            base_x + horizontal_middle - arrow_middle,
            base_y - arrow_middle,
            "-45deg",
        ),
        PopoverPosition::Left => (
            base_x + popover_width - arrow_middle,
            base_y + vertical_middle - arrow_middle,
            "45deg",
        ),
        PopoverPosition::Right => (
            base_x - arrow_middle,
            base_y + vertical_middle - arrow_middle,
            "-135deg",
        ),
        PopoverPosition::TopStart => (
            base_x + corner_offset,
            base_y - arrow_middle + popover_height,
            "135deg",
        ),
        PopoverPosition::TopEnd => (
            base_x + popover_width - corner_offset,
            base_y - arrow_middle + popover_height,
            "135deg",
        ),
        PopoverPosition::LeftStart => (
            base_x + popover_width - arrow_middle,
            base_y + corner_offset,
            "45deg",
        ),
        PopoverPosition::LeftEnd => (
            base_x + popover_width - arrow_middle,
            base_y - corner_offset,
            "45deg",
        ),
        PopoverPosition::RightStart => (base_x - arrow_middle, base_y + corner_offset, "-135deg"),
        PopoverPosition::RightEnd => (base_x - arrow_middle, base_y - corner_offset, "-135deg"),
        PopoverPosition::BottomStart => (base_x + corner_offset, base_y - arrow_middle, "-45deg"),
        PopoverPosition::BottomEnd => (
            base_x + popover_width - corner_offset,
            base_y - arrow_middle,
            "-45deg",
        ),
    };

    arrow_style.set_property("left", format!("{left}px").as_str());
    arrow_style.set_property("top", format!("{top}px").as_str());
    arrow_style.set_property("transform", format!("rotate({rotation})").as_str());
}

/// Finds an ideal collision-free area next to [trigger] to place [popover].
/// returns the chosen position, and absolute coordinates [popover] needs to be placed at for this position.
fn find_popover_abs_position(
    preferred_position: PopoverPosition,
    popover: &web_sys::Element,
    trigger: &web_sys::Element,
    show_arrow: bool,
) -> (Option<PopoverPosition>, RelativePosition) {
    let fallback = (
        None,
        RelativePosition {
            horizontal_offset: None,
            vertical_offset: None,
        },
    );
    let popover_rect = get_true_bb(&popover);
    let trigger_rect = get_true_bb(&trigger);

    // Popover trigger element, we normally display next to it.
    let trigger_x = trigger_rect.x();
    let trigger_y = trigger_rect.y();
    let trigger_width = trigger_rect.width();
    let trigger_height = trigger_rect.height();

    let popover_width = popover_rect.width();
    let popover_height = popover_rect.height();
    debug_log!("trigger_x: {trigger_x:?}");
    debug_log!("trigger_y: {trigger_y:?}");
    debug_log!("trigger_width: {trigger_width:?}");
    debug_log!("trigger_height: {trigger_height:?}");
    debug_log!("popover_width: {popover_width:?}");
    debug_log!("popover_height: {popover_height:?}");

    let arrow_bump = if show_arrow { 6.0 } else { 0.0 };

    let Some(window_width) = window_inner_width() else {
        debug_warn!("No window width, falling back");

        return fallback;
    };
    let Some(window_horizontal_min) = window().scroll_x().ok() else {
        debug_warn!("No window scroll, falling back");
        return fallback;
    };
    let window_horizontal_max = window_horizontal_min + window_width;

    let Some(window_height) = window_inner_height() else {
        debug_warn!("No window height, falling back");

        return fallback;
    };
    let Some(window_vertical_min) = window().scroll_y().ok() else {
        debug_warn!("No window scroll, falling back");
        return fallback;
    };
    let window_vertical_max = window_vertical_min + window_height;

    let top_top_is_open = window_vertical_min < trigger_y - (popover_height + arrow_bump);
    let bot_bot_is_open =
        window_vertical_max > trigger_y + trigger_height + (popover_height + arrow_bump);
    let left_left_is_open = window_horizontal_min < trigger_x - (popover_width + arrow_bump);
    debug_log!("Left is open: {left_left_is_open}");
    let right_right_is_open =
        window_horizontal_max > trigger_x + trigger_width + (popover_width + arrow_bump);

    let horizontal_start_is_open = window_horizontal_max > trigger_x + popover_width;
    let horizontal_end_is_open = window_horizontal_min < trigger_x + trigger_width - popover_width;
    let vertical_start_is_open = window_vertical_max > trigger_y + popover_height;
    let vertical_end_is_open = window_vertical_min < trigger_y + trigger_height - popover_height;

    // * Popovers should not be wider than the page_width.
    // * Assumes the trigger is not half-onscreen.
    let possible_positions = POPOVER_POSITIONS
        .into_iter()
        .filter(|position| match position {
            // Collision checks
            PopoverPosition::TopStart => top_top_is_open && horizontal_start_is_open,
            PopoverPosition::Top => top_top_is_open,
            PopoverPosition::TopEnd => top_top_is_open && horizontal_end_is_open,

            PopoverPosition::BottomStart => bot_bot_is_open && horizontal_start_is_open,
            PopoverPosition::Bottom => bot_bot_is_open,
            PopoverPosition::BottomEnd => bot_bot_is_open && horizontal_end_is_open,

            PopoverPosition::LeftStart => left_left_is_open && vertical_start_is_open,
            PopoverPosition::Left => left_left_is_open,
            PopoverPosition::LeftEnd => left_left_is_open && vertical_end_is_open,

            PopoverPosition::RightStart => right_right_is_open && vertical_start_is_open,
            PopoverPosition::Right => right_right_is_open,
            PopoverPosition::RightEnd => right_right_is_open && vertical_end_is_open,
        });

    let mut best_position = None;
    for (i, position) in possible_positions.enumerate() {
        debug_log!("Considering {i} - {position:?}");
        if *position == preferred_position {
            debug_log!("Picked as perfect {position:?}");
            best_position = Some(position);
            break;
        } else if *position == preferred_position.mirrored() {
            debug_log!("Picked as mirror {position:?}");
            best_position = Some(position);
        } else if i == 0 {
            debug_log!("Picked as fallback {position:?}");
            best_position = Some(position);
        }
    }

    debug_log!("Best position {:?}", best_position);
    if let Some(best_position) = best_position {
        // Map to absolute position
        let (horizontal_offset, vertical_offset) = match best_position {
            PopoverPosition::TopStart => (
                trigger_rect.left() as u32,
                (trigger_rect.top() - popover_height - arrow_bump) as u32,
            ),
            PopoverPosition::Top => (
                (trigger_rect.left() + (trigger_width - popover_width) / 2.0) as u32,
                (trigger_rect.top() - popover_height - arrow_bump) as u32,
            ),
            PopoverPosition::TopEnd => (
                (trigger_rect.right() - popover_width) as u32,
                (trigger_rect.top() - popover_height - arrow_bump) as u32,
            ),

            PopoverPosition::BottomStart => (
                trigger_rect.left() as u32,
                (trigger_rect.bottom() + arrow_bump) as u32,
            ),
            PopoverPosition::Bottom => (
                (trigger_rect.left() + (trigger_width - popover_width) / 2.0) as u32,
                (trigger_rect.bottom() + arrow_bump) as u32,
            ),
            PopoverPosition::BottomEnd => (
                (trigger_rect.right() - popover_width) as u32,
                (trigger_rect.bottom() + arrow_bump) as u32,
            ),

            PopoverPosition::LeftStart => (
                (trigger_rect.left() - popover_width - arrow_bump) as u32,
                trigger_y as u32,
            ),
            PopoverPosition::Left => (
                (trigger_rect.left() - popover_width - arrow_bump) as u32,
                (trigger_y + (trigger_height - popover_height) / 2.0) as u32,
            ),
            PopoverPosition::LeftEnd => (
                (trigger_rect.left() - popover_width - arrow_bump) as u32,
                (trigger_rect.bottom() - popover_height) as u32,
            ),

            PopoverPosition::RightStart => {
                ((trigger_rect.right() + arrow_bump) as u32, trigger_y as u32)
            }
            PopoverPosition::Right => (
                (trigger_rect.right() + arrow_bump) as u32,
                (trigger_y + (trigger_height - popover_height) / 2.0) as u32,
            ),
            PopoverPosition::RightEnd => (
                (trigger_rect.right() + arrow_bump) as u32,
                (trigger_rect.bottom() - popover_height) as u32,
            ),
        };

        return (
            Some(*best_position),
            RelativePosition {
                horizontal_offset: Some(HorizontalOffset::Left(horizontal_offset)),
                vertical_offset: Some(VerticalOffset::Top(vertical_offset)),
            },
        );
    };

    return fallback;
}

fn window_inner_width() -> Option<f64> {
    window().inner_width().ok()?.as_f64()
}

fn window_inner_height() -> Option<f64> {
    window().inner_height().ok()?.as_f64()
}

#[derive(Debug, Default, Clone)]
pub enum PopoverSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl PopoverSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Clone)]
pub enum PopoverAppearance {
    Default,
    Inverted,
}

impl PopoverAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverAppearance::Inverted => "inverted",
            PopoverAppearance::Default => "default",
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum PopoverTriggerType {
    #[default]
    Hover,
    Click,
}

impl Copy for PopoverTriggerType {}

/// Keep in sync with POPOVER_POSITIONS
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PopoverPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
}
impl PopoverPosition {
    fn mirrored(&self) -> PopoverPosition {
        match self {
            PopoverPosition::Top => Self::Bottom,
            PopoverPosition::Bottom => Self::Bottom,
            PopoverPosition::Left => Self::Right,
            PopoverPosition::Right => Self::Left,
            PopoverPosition::TopStart => Self::BottomStart,
            PopoverPosition::TopEnd => Self::BottomEnd,
            PopoverPosition::LeftStart => Self::RightStart,
            PopoverPosition::LeftEnd => Self::RightEnd,
            PopoverPosition::RightStart => Self::LeftStart,
            PopoverPosition::RightEnd => Self::LeftEnd,
            PopoverPosition::BottomStart => Self::TopStart,
            PopoverPosition::BottomEnd => Self::TopEnd,
        }
    }
}

const POPOVER_POSITIONS: &[PopoverPosition] = &[
    PopoverPosition::Top,
    PopoverPosition::Bottom,
    PopoverPosition::Left,
    PopoverPosition::Right,
    PopoverPosition::TopStart,
    PopoverPosition::TopEnd,
    PopoverPosition::LeftStart,
    PopoverPosition::LeftEnd,
    PopoverPosition::RightStart,
    PopoverPosition::RightEnd,
    PopoverPosition::BottomStart,
    PopoverPosition::BottomEnd,
];

#[slot]
pub struct PopoverTrigger<T> {
    children: TypedChildren<T>,
}
