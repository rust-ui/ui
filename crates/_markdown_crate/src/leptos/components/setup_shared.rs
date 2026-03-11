use crate::leptos::components::md_headings::{md_h2, md_h3, md_h4, md_h5, md_h6};
use crate::leptos::leptos_converter::{MdComponentProps, MdComponents};
use crate::ui::callout::callout_md;
use crate::ui::steps::{Step, StepProps, Steps, StepsProps};

pub fn setup_shared() -> MdComponents {
    let mut components = MdComponents::new();

    components.add_with_props("Steps", Steps, |props: MdComponentProps| StepsProps { children: props.children });
    components.add_with_props("Step", Step, |props: MdComponentProps| StepProps { children: props.children });

    components.add_component_with_anchor_support("Callout", callout_md);

    // Heading components with clickable anchor links
    components.add_component_with_anchor_support("h2", md_h2);
    components.add_component_with_anchor_support("h3", md_h3);
    components.add_component_with_anchor_support("h4", md_h4);
    components.add_component_with_anchor_support("h5", md_h5);
    components.add_component_with_anchor_support("h6", md_h6);

    //
    //
    components
}
