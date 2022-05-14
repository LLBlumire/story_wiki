use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub mod continuity_picker;
pub mod release_picker;

#[derive(PartialEq, Debug)]
pub enum OptionSegment {
    OptionGroup {
        label: String,
        options: Vec<SingleOption>,
        hidden: bool,
    },
    SingleOption(SingleOption),
}
impl OptionSegment {
    fn html(&self) -> Html {
        match self {
            OptionSegment::OptionGroup {
                label,
                options,
                hidden,
            } => html! {
                <optgroup label={label.to_string()} hidden={*hidden}>
                    { for options.iter().map(SingleOption::html) }
                </optgroup>
            },
            OptionSegment::SingleOption(single_option) => single_option.html(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct SingleOption {
    pub value: String,
    pub display: String,
    pub selected: bool,
    pub hidden: bool,
}
impl SingleOption {
    fn html(&self) -> Html {
        html! {
            <option
                value={self.value.to_string()}
                selected={self.selected}
                hidden={self.hidden}
            >
                {&self.display}
            </option>
        }
    }
}

pub struct PickerFeed {
    /// The value the picker will callback with
    pub value: String,
    /// The display value of the option
    pub display: String,
    /// True if this option is currently selected
    pub selected: bool,
    /// True if this option is to be hidden, contributing width to the element by not content
    pub hidden: bool,
    /// None to continue previous group, Some(None) to exit group, Some(Some(v)) to start group v
    pub new_group: Option<Option<String>>,
}

impl FromIterator<PickerFeed> for Vec<OptionSegment> {
    fn from_iter<T: IntoIterator<Item = PickerFeed>>(iter: T) -> Self {
        iter.into_iter()
            .fold(
                Vec::<OptionSegment>::new(),
                |mut vec,
                 PickerFeed {
                     value,
                     display,
                     hidden,
                     selected,
                     new_group,
                 }| {
                    let option = SingleOption {
                        value,
                        display,
                        selected,
                        hidden,
                    };
                    match (vec.last_mut(), new_group) {
                        (Some(OptionSegment::OptionGroup { options, .. }), None) => {
                            options.push(option)
                        }
                        (_, Some(Some(label))) => vec.push(OptionSegment::OptionGroup {
                            hidden,
                            label,
                            options: vec![option],
                        }),
                        _ => vec.push(OptionSegment::SingleOption(option)),
                    }
                    vec
                },
            )
            .into_iter()
            .collect()
    }
}

#[derive(PartialEq, Properties)]
pub struct PickerProps {
    #[prop_or_default]
    pub options: Vec<OptionSegment>,
    #[prop_or_default]
    pub onpick: Callback<String>,
}

#[function_component]
pub fn Picker(props: &PickerProps) -> Html {
    log::trace!("Rendering Picker");
    let select_ref = use_node_ref();
    let select_ref_inner = select_ref.clone();
    let onpick = props.onpick.clone();
    let onchange = Callback::from(move |_| {
        let value = select_ref_inner
            .cast::<HtmlSelectElement>()
            .unwrap()
            .value();
        onpick.emit(value)
    });
    let select_ref_inner2 = select_ref.clone();
    let selected = {
        props
            .options
            .iter()
            .find_map(|segment| match segment {
                OptionSegment::OptionGroup { options, .. } => options
                    .iter()
                    .find_map(|option| Some(option).filter(|option| option.selected)),
                OptionSegment::SingleOption(option) => {
                    Some(option).filter(|option| option.selected)
                }
            })
            .map(|option| option.value.clone())
    };
    use_effect(move || {
        if let Some(selected) = selected {
            select_ref_inner2
                .cast::<HtmlSelectElement>()
                .unwrap()
                .set_value(&selected);
        }
        || ()
    });
    html! {
        <select {onchange} ref={select_ref}>
            { for props.options.iter().map(OptionSegment::html) }
        </select>
    }
}
