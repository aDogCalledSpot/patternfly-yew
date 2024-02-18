use std::rc::Rc;

use yew::prelude::*;

use super::composable::*;

pub trait TableColumns: std::fmt::Display + PartialEq + Clone {
    fn iter_cols() -> impl Iterator<Item = Self>;

    // fn colspan(&self) -> Option<usize> {
    //     None
    // }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowExpandArgs {
    pub expanded: bool,
    pub ontoggle: Callback<()>,
    pub content: Rc<Vec<crate::prelude::Span>>,
}

impl From<RowExpandArgs> for ExpandParams {
    fn from(value: RowExpandArgs) -> Self {
        Self {
            r#type: ExpandType::Row,
            expanded: value.expanded,
            ontoggle: value.ontoggle,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NewRowProperties<C: TableColumns> {
    #[prop_or_default]
    pub class: Classes,
    pub render_cells: Callback<C, Html>,
    #[prop_or_default]
    pub center_cells: Option<Callback<C, bool>>,
    #[prop_or_default]
    pub expandable: Option<RowExpandArgs>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub selected: bool,
}

#[function_component(NewRow)]
pub fn new_row<C: TableColumns>(props: &NewRowProperties<C>) -> Html {
    html! {
        <>
            <TableRow
                class={props.class.clone()}
                selected={props.selected}
                onclick={props.onclick.clone()}
            >
                // The toggle
                if let Some(expandable) = props.expandable.as_ref() {
                    <TableData expandable={ExpandParams {
                        r#type: ExpandType::Row,
                        expanded: expandable.expanded,
                        ontoggle: expandable.ontoggle.clone()
                    }} />
                }
                {
                    for C::iter_cols().map(|col| {
                        html! {
                            <TableData
                                center={
                                    let col = col.clone();
                                    props.center_cells.as_ref().is_some_and(move |f| f.emit(col))
                                }
                                data_label={format!("{col}")}
                            >
                                { props.render_cells.emit(col) }
                            </TableData>
                        }
                    })
                }
            </TableRow>
            if let Some(&RowExpandArgs { ref content, expanded, .. }) = props.expandable.as_ref() {
                <TableRow expandable=true {expanded}>
                    {
                        for content.iter().map(|cell| html! {
                            <TableData span_modifiers={cell.modifiers.clone()} colspan={cell.cols}>
                                <ExpandableRowContent>{ cell.content.clone() }</ExpandableRowContent>
                            </TableData>
                        })
                    }
                </TableRow>
            }
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct NewTableHeaderProperties {

}
