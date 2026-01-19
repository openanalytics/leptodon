use leptos_struct_table::DragHandler;

pub mod grouping;

pub struct StyledHeadDragHandler;

impl<Column> DragHandler<Column> for StyledHeadDragHandler
where
    Column: Clone + PartialEq + Send + Sync + 'static,
{
    fn grabbed_class(&self) -> &'static str {
        "outline outline-blue-500 outline-dashed -outline-offset-1 bg-blue-500/10"
    }

    fn hover_left_class(&self) -> &'static str {
        "relative border-l-2 border-blue-500 after:content-[''] after:absolute after:left-0 after:top-0 after:w-0 after:h-0 after:border-l-[6px] after:border-l-transparent after:border-r-[6px] after:border-r-transparent after:border-t-[8px] after:border-t-blue-500 after:-translate-x-[7px]"
    }

    fn hover_right_class(&self) -> &'static str {
        "relative border-r-2 border-blue-500 after:content-[''] after:absolute after:right-0 after:top-0 after:w-0 after:h-0 after:border-r-[6px] after:border-r-transparent after:border-l-[6px] after:border-l-transparent after:border-t-[8px] after:border-t-blue-500 after:translate-x-[7px]"
    }
}
